use chrono::prelude::*;
use clap::Parser;
use html_escape;
use owo_colors::{AnsiColors, OwoColorize};
use serde::{Deserialize, Serialize};
use std::cmp;
use unicode_width::UnicodeWidthStr;

#[derive(Deserialize, Serialize, Debug)]
struct Channel {
    yt_channel_id: String,
    name: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Live {
    yt_video_key: String,
    title: String,
    live_schedule: String,
    live_start: Option<String>,
    live_end: Option<String>,
    channel: Channel,
}

impl Live {
    fn is_live(&self) -> bool {
        self.live_start.is_some() && self.live_end.is_none()
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    live: Vec<Live>,
    upcoming: Vec<Live>,
    ended: Vec<Live>,
}

fn get_live_timestamp_str(live: &Live) -> String {
    if live.live_start.is_some() {
        &live.live_start.as_ref().unwrap()
    } else {
        &live.live_schedule
    }
    .to_string()
}

fn print_lives(data: APIResponse, args: Args) {
    let mut lives = data.ended;
    lives.extend(data.live);
    lives.extend(data.upcoming);

    // If --live flag is enabled, filter out streams that are not currently live
    if args.live {
        lives.retain(|live| live.is_live());
    }

    // Sort the streams by timestamp
    lives.sort_by(|a, b| {
        let timestamp_a = get_live_timestamp_str(a);
        let timestamp_b = get_live_timestamp_str(b);

        timestamp_a.cmp(&timestamp_b)
    });

    // Get the width of the widest channel name
    let max_name_width = lives.iter().fold(0, |acc, live| {
        cmp::max(UnicodeWidthStr::width(&*live.channel.name), acc)
    });

    for live in lives.iter() {
        let name_width = UnicodeWidthStr::width(&*live.channel.name);

        let white_space = " ".repeat(max_name_width - name_width);

        // If live_start has some value, live stream has started
        let has_started = live.live_start.is_some();
        let has_ended = live.live_end.is_some();

        let timestamp = get_live_timestamp_str(live);

        let fixed_offset = match DateTime::parse_from_rfc3339(&timestamp) {
            Ok(t) => t,
            Err(_) => return eprintln!("{}", "Unable to parse stream timestamp".red()),
        };

        let local_time: DateTime<Local> = DateTime::from(fixed_offset);

        // Don't show streams that are more than 12 hours "late"
        let is_late = !has_started && Local::now().timestamp() - local_time.timestamp() > 43200;
        if is_late {
            continue;
        }

        let formatted = format!(
            "{}  {}{}  https://youtu.be/{}  {}",
            local_time.format("%H:%M"),
            live.channel.name,
            white_space,
            live.yt_video_key,
            // Some characters are encoded in the api response (like " -> &quot;) so decode them
            html_escape::decode_html_entities(&live.title)
        );

        if has_ended {
            println!("{}", formatted.bright_black());
        } else if has_started {
            println!("{}", formatted.magenta());
        } else {
            println!("{}", formatted.white());
        }
    }
}

fn fetch_data() -> Result<APIResponse, String> {
    let resp  = match reqwest::blocking::get("https://api.holotools.app/v1/live?max_upcoming_hours=24&lookback_hours=0&hide_channel_desc=1") {
        Ok(resp) => resp,
        Err(err) => return Err(format!("Unable to connect to the API: {}", err))
    };

    match resp.status() {
        reqwest::StatusCode::OK => (),
        _ => {
            return Err(format!(
                "Received an invalid response from the API: {}",
                resp.status()
            ))
        }
    };

    let parsed = match resp.json::<APIResponse>() {
        Ok(parsed) => parsed,
        Err(_) => return Err("Unable to parse response JSON".to_string()),
    };

    Ok(parsed)
}

const BIBI_ASCII: &str = "
  d8b, |        |        | ,d8b
 88888,|        |        |,88888
`88888.|        |        |.88888'
  88'  | b,   ,d|b,   ,d |  `88
       |  `8.8' | `8.8'
       |    '   |   '
";

fn print_bibi() {
    let colors = [
        AnsiColors::Magenta,
        AnsiColors::Yellow,
        AnsiColors::Magenta,
        AnsiColors::Yellow,
    ];
    // Split Bibi ascii into lines while keeping line breaks
    for line in BIBI_ASCII.split_inclusive('\n') {
        // Split the lines on "|" and color the splits
        for (text, color) in line.split('|').zip(colors.iter().copied()) {
            print!("{}", text.color(color).bold());
        }
    }
    println!();
}

#[derive(Parser)]
#[clap(about, long_about = None, version )]
struct Args {
    #[clap(short, long, help = "Print a cute Bibi ascii art")]
    ascii: bool,

    #[clap(short, long, help = "Print only streams that are currently live")]
    live: bool,
}

fn main() {
    let args = Args::parse();

    if args.ascii {
        print_bibi();
    } else {
        let data = match fetch_data() {
            Ok(resp) => resp,
            Err(err) => return eprintln!("{}", err.red()),
        };
        print_lives(data, args);
    }
}
