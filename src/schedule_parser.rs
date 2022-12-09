use chrono::prelude::*;
use chrono_tz::Asia::Tokyo;
use chrono_tz::Tz;
use futures::stream::*;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

use crate::channels::{Branch, Channel, CHANNELS};
use crate::cli::Args;
use crate::config::Config;

#[derive(Clone, Debug)]
pub enum LiveStreamStatus {
    Ended,
    Live,
    Upcoming,
}

#[derive(Clone, Debug)]
pub struct HoloduleData {
    id: String,
    time: DateTime<FixedOffset>,
    status: LiveStreamStatus,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct OEmbedData {
    title: String,
    author_name: String,
    author_url: String,
}

pub struct LiveStream {
    pub id: String,
    pub title: String,
    pub author_name: String,
    pub author_id: String,
    pub status: LiveStreamStatus,
    pub time: DateTime<FixedOffset>,
}

async fn fetch_html() -> Result<String, String> {
    let resp = match reqwest::get("https://schedule.hololive.tv/").await {
        Ok(resp) => resp,
        Err(err) => {
            return Err(format!(
                "Unable to connect to https://schedule.hololive.tv: {}",
                err
            ))
        }
    };

    match resp.status() {
        reqwest::StatusCode::OK => (),
        _ => return Err(format!("Received an invalid response: {}", resp.status())),
    };

    let body = match resp.text().await {
        Ok(resp) => resp,
        Err(_) => return Err("Unable to parse response text".to_owned()),
    };

    Ok(body)
}

async fn fetch_oembed_data(holodule_data: Vec<HoloduleData>) -> Result<Vec<LiveStream>, String> {
    let fetches = futures::stream::iter(&holodule_data)
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
        .await;

    let streams = holodule_data
        .iter()
        .zip(fetches.iter())
        .filter_map(|(holodule, oembed)| {
            if oembed.is_err() {
                return None;
            }

            let oembed = oembed.as_ref().unwrap();

            let (_, author_id) = oembed.author_url.rsplit_once('/').unwrap();

            Some(LiveStream {
                id: holodule.id.to_owned(),
                title: oembed.title.to_owned(),
                author_name: oembed.author_name.to_owned(),
                author_id: author_id.to_owned(),
                status: holodule.status.to_owned(),
                time: holodule.time.to_owned(),
            })
        })
        .collect::<Vec<LiveStream>>();

    Ok(streams)
}

fn parse_html(html: &str) -> Result<Vec<HoloduleData>, String> {
    let document = Html::parse_document(html);

    let first_date: Vec<u32> = document
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
        .collect();

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

            let (_, video_id) = url.rsplit_once("?v=").unwrap();

            let live = element.value().attr("style")?.contains("border: 3px red");

            let selector = Selector::parse("div.datetime").unwrap();
            let time_str = element
                .select(&selector)
                .next()?
                .text()
                .collect::<String>()
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>();

            let naive_time = NaiveTime::parse_from_str(&*time_str, "%H:%M").unwrap();

            if naive_time_prev.is_some() && naive_time_prev.unwrap() > naive_time {
                naive_date = naive_date.succ()
            }

            naive_time_prev = Some(naive_time);

            let jst_offset = 9 * 3600;
            let streamtime = FixedOffset::east(jst_offset)
                .ymd(naive_date.year(), naive_date.month(), naive_date.day())
                .and_hms(naive_time.hour(), naive_time.minute(), 0);

            let status = if live {
                LiveStreamStatus::Live
            } else if streamtime.timestamp() + 15 * 60 < jst_now.timestamp()
                || streamtime.minute() % 5 != 0
            {
                LiveStreamStatus::Ended
            } else {
                LiveStreamStatus::Upcoming
            };

            let live_stream = HoloduleData {
                id: video_id.to_owned(),
                time: streamtime,
                status,
            };

            Some(live_stream)
        })
        .collect();

    Ok(live_streams)
}

fn vector_contains_channel(channel_list: &[String], author: &Channel) -> bool {
    channel_list.iter().any(|s| {
        if s == author.id {
            return true;
        }

        if let Some(handle) = author.handle {
            if s.to_lowercase() == handle.to_lowercase() {
                return true;
            }
        }

        false
    })
}

pub async fn get_schedule(args: &Args, cfg: &Config) -> Result<Vec<LiveStream>, String> {
    let body = fetch_html().await?;

    let mut lives = parse_html(&body)?;

    if !args.all {
        if !args.live && !args.ended && !args.upcoming {
            // If no args are selected, don't show "Ended"
            lives.retain(|live| !matches!(live.status, LiveStreamStatus::Ended));
        } else {
            lives.retain(|stream| match stream.status {
                LiveStreamStatus::Live => args.live,
                LiveStreamStatus::Ended => args.ended,
                LiveStreamStatus::Upcoming => args.upcoming,
            });
        }
    }

    let mut live_streams = fetch_oembed_data(lives).await?;

    if !cfg.branches.hololive || !cfg.branches.holostars || !cfg.channels.exclude.is_empty() {
        live_streams.retain(|stream| {
            match CHANNELS
                .iter()
                .find(|&c| c.handle.as_ref().unwrap_or(&c.id) == &stream.author_id)
            {
                Some(author) => {
                    if vector_contains_channel(&cfg.channels.include, author) {
                        return true;
                    }

                    if vector_contains_channel(&cfg.channels.exclude, author) {
                        return false;
                    }

                    match author.branch {
                        Branch::Hololive => cfg.branches.hololive,
                        Branch::Holostars => cfg.branches.holostars,
                    }
                }
                None => true,
            }
        });
    }

    Ok(live_streams)
}
