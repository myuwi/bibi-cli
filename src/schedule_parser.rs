use chrono::prelude::*;
use chrono_tz::{Asia::Tokyo, Tz};
use futures::stream::*;
use reqwest::StatusCode;
use scraper::{Html, Selector};
use thiserror::Error;

use crate::{
    cli::Args,
    config::Config,
    types::{Branch, HoloduleData, LiveStream, LiveStreamStatus, OEmbedData},
};

#[derive(Error, Debug)]
pub enum ScheduleParserError {
    #[error("Unable to connect to {0}")]
    ConnectionError(String),
    #[error("Received an invalid response: {0}")]
    InvalidRespose(u16),
    #[error("Unable to parse response text")]
    ParseError,
}

const SCHEDULE_URL: &str = "https://schedule.hololive.tv/";

async fn fetch_html() -> Result<String, ScheduleParserError> {
    let resp = reqwest::get(SCHEDULE_URL)
        .await
        .map_err(|_| ScheduleParserError::ConnectionError(SCHEDULE_URL.to_owned()))?;

    if resp.status() != StatusCode::OK {
        return Err(ScheduleParserError::InvalidRespose(resp.status().as_u16()));
    }

    let body = resp
        .text()
        .await
        .map_err(|_| ScheduleParserError::ParseError)?;

    Ok(body)
}

fn parse_html(html: &str) -> Result<Vec<HoloduleData>, ScheduleParserError> {
    let document = Html::parse_document(html);

    let first_date = document
        .select(&Selector::parse(".holodule.navbar-text").unwrap())
        .next()
        .unwrap()
        .text()
        .collect::<String>()
        .split('(')
        .next()
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .split('/')
        .map(|element| element.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let jst_now: DateTime<Tz> = Utc::now().with_timezone(&Tokyo);
    let mut naive_date = NaiveDate::from_ymd(jst_now.year(), jst_now.month(), jst_now.day());

    for _ in 0..3 {
        if naive_date.month() == first_date[0] && naive_date.day() == first_date[1] {
            break;
        } else {
            naive_date = naive_date.pred();
        }
    }

    let mut naive_time_prev: Option<NaiveTime> = None;

    let live_streams = document
        .select(&Selector::parse("a.thumbnail").unwrap())
        .filter_map(|element| {
            let url = element.value().attr("href")?;

            if !url.contains("youtube.com") {
                return None;
            }

            let (_, video_id) = url.rsplit_once("?v=")?;

            let live = element.value().attr("style")?.contains("border: 3px red");

            // Get stream start time string
            let selector = Selector::parse("div.datetime").unwrap();
            let time_str = element
                .select(&selector)
                .next()?
                .text()
                .collect::<String>()
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>();

            // Create naive time from start time string
            let naive_time = NaiveTime::parse_from_str(&time_str, "%H:%M").unwrap();

            // TODO: Make sure there aren't edge cases where this doesn't work
            // If time is less than the last time assume it is the next day so update the date
            if naive_time_prev.is_some() && naive_time_prev.unwrap() > naive_time {
                naive_date = naive_date.succ()
            }

            naive_time_prev = Some(naive_time);

            // Create a DateTime for the stream
            let jst_offset = 9 * 3600;
            let stream_time = FixedOffset::east_opt(jst_offset)
                .unwrap()
                .ymd(naive_date.year(), naive_date.month(), naive_date.day())
                .and_hms(naive_time.hour(), naive_time.minute(), 0);

            let status = if live {
                LiveStreamStatus::Live
            } else if stream_time.timestamp() + 15 * 60 < jst_now.timestamp()
                || stream_time.minute() % 5 != 0
            {
                LiveStreamStatus::Ended
            } else {
                LiveStreamStatus::Upcoming
            };

            let live_stream = HoloduleData {
                id: video_id.to_owned(),
                time: stream_time,
                status,
            };

            Some(live_stream)
        })
        .collect();

    Ok(live_streams)
}

async fn fetch_oembed_data(
    holodule_data: &Vec<HoloduleData>,
) -> Vec<Result<OEmbedData, Box<dyn std::error::Error>>> {
    futures::stream::iter(holodule_data)
        .map(|live_stream| async {
            let data = reqwest::get(format!(
                "https://www.youtube.com/oembed?url=https://youtu.be/{}&format=json",
                live_stream.id
            ))
            .await?
            .json::<OEmbedData>()
            .await?;

            Ok(data)
        })
        .buffered(16)
        .collect::<Vec<Result<OEmbedData, Box<dyn std::error::Error>>>>()
        .await
}

pub async fn get_schedule(
    args: &Args,
    cfg: &Config,
) -> Result<Vec<LiveStream>, ScheduleParserError> {
    let html = fetch_html().await?;
    let mut holodule_data = parse_html(&html)?;

    if !args.all {
        if !args.live && !args.ended && !args.upcoming {
            // If no args are selected, don't show "Ended"
            holodule_data.retain(|live| !matches!(live.status, LiveStreamStatus::Ended));
        } else {
            holodule_data.retain(|stream| match stream.status {
                LiveStreamStatus::Live => args.live,
                LiveStreamStatus::Ended => args.ended,
                LiveStreamStatus::Upcoming => args.upcoming,
            });
        }
    }

    let oembed_data = fetch_oembed_data(&holodule_data).await;

    let mut live_streams = holodule_data
        .iter()
        .zip(oembed_data.iter())
        .filter_map(|(holodule, oembed)| match oembed {
            Ok(oembed) => {
                let (_, author_handle) = oembed.author_url.rsplit_once('/').unwrap();

                Some(LiveStream {
                    author_name: oembed.author_name.to_owned(),
                    author_handle: author_handle.to_owned(),
                    id: holodule.id.to_owned(),
                    title: oembed.title.to_owned(),
                    status: holodule.status.to_owned(),
                    time: holodule.time.to_owned(),
                })
            }
            _ => None,
        })
        .collect::<Vec<LiveStream>>();

    if !cfg.branches.hololive || !cfg.branches.holostars || !cfg.channels.exclude.is_empty() {
        live_streams.retain(|stream| {
            if cfg.channels.include.contains(&stream.author_handle) {
                return true;
            }

            if cfg.channels.exclude.contains(&stream.author_handle) {
                return false;
            }

            match stream.get_branch() {
                Branch::Hololive => cfg.branches.hololive,
                Branch::Holostars => cfg.branches.holostars,
            }
        });
    }

    Ok(live_streams)
}
