use chrono::prelude::*;
use chrono_tz::Asia::Tokyo;
use chrono_tz::Tz;
use futures::stream::*;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

use crate::cli::Args;

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
    pub author_url: String,
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
    let holodule_data_clone = holodule_data.clone();

    let fetches = futures::stream::iter(holodule_data_clone)
        .map(|live_stream| async move {
            let resp = match reqwest::get(format!(
                "https://www.youtube.com/oembed?url=https://youtu.be/{}&format=json",
                live_stream.id
            ))
            .await
            {
                Ok(resp) => match resp.json::<OEmbedData>().await {
                    Ok(resp) => resp,
                    Err(_) => OEmbedData::default(),
                },
                Err(_) => OEmbedData::default(),
            };
            resp
        })
        .buffered(16)
        .collect::<Vec<OEmbedData>>()
        .await;

    let streams = holodule_data
        .iter()
        .zip(fetches.iter())
        .filter_map(|(holodule, oembed)| {
            if oembed.author_url == "" {
                return None;
            }

            Some(LiveStream {
                id: holodule.id.to_owned(),
                title: oembed.title.to_owned(),
                author_name: oembed.author_name.to_owned(),
                author_url: oembed.author_url.to_owned(),
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
        .split("(")
        .next()
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .split("/")
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

    let mut last_naive_time: Option<NaiveTime> = None;

    let live_streams: Vec<HoloduleData> = document
        .select(&Selector::parse("a.thumbnail").unwrap())
        .filter_map(|element| {
            let url = element.value().attr("href")?;

            if !url.contains("youtube.com") {
                return None;
            }

            let video_id = url.split("?v=").last().unwrap();

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

            if last_naive_time.is_some() && last_naive_time.unwrap() > naive_time {
                naive_date = naive_date.succ()
            }

            last_naive_time = Some(naive_time);

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

pub async fn get_schedule(args: &Args) -> Result<Vec<LiveStream>, String> {
    let body = match fetch_html().await {
        Ok(resp) => resp,
        Err(err) => return Err(err),
    };

    let mut lives = parse_html(&body).unwrap();

    // If no args are selected, don't show "Ended"
    if !args.live && !args.ended && !args.upcoming {
        lives.retain(|live| !matches!(live.status, LiveStreamStatus::Ended));
    } else {
        lives = lives
            .into_iter()
            .filter(|stream| {
                if args.live && matches!(stream.status, LiveStreamStatus::Live) {
                    return true;
                } else if args.ended && matches!(stream.status, LiveStreamStatus::Ended) {
                    return true;
                } else if args.upcoming && matches!(stream.status, LiveStreamStatus::Upcoming) {
                    return true;
                }
                false
            })
            .collect::<Vec<_>>();
    }

    let live_streams = match fetch_oembed_data(lives).await {
        Ok(parsed) => parsed,
        Err(err) => return Err(err),
    };

    Ok(live_streams)
}
