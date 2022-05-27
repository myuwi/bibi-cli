mod schedule_parser;
use schedule_parser::{LiveStream, LiveStreamStatus};

mod cli;
use cli::Args;

use chrono::prelude::*;
use owo_colors::{AnsiColors, OwoColorize};
use std::cmp;
use unicode_width::UnicodeWidthStr;

fn print_lives(lives: &Vec<LiveStream>, _args: &Args) {
    // Get the width of the widest channel name
    let max_name_width = lives.iter().fold(0, |acc, live| {
        cmp::max(UnicodeWidthStr::width(&*live.author_name), acc)
    });

    for live in lives.iter() {
        let name_width = UnicodeWidthStr::width(&*live.author_name);

        let white_space = " ".repeat(max_name_width - name_width);

        let local_time: DateTime<Local> = DateTime::from(live.time);

        let formatted = format!(
            "{}  {}{}  https://youtu.be/{}  {}",
            local_time.format("%H:%M"),
            live.author_name,
            white_space,
            live.id,
            live.title
        );

        match live.status {
            LiveStreamStatus::Ended => println!("{}", formatted.bright_black()),
            LiveStreamStatus::Live => println!("{}", formatted.magenta()),
            LiveStreamStatus::Upcoming => println!("{}", formatted.white()),
        }
    }
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

#[tokio::main]
async fn main() {
    let args = Args::new();

    if args.ascii {
        print_bibi();
    } else {
        let lives = match schedule_parser::get_schedule(&args).await {
            Ok(resp) => resp,
            Err(err) => return eprintln!("{}", err.red()),
        };
        print_lives(&lives, &args);
    }
}
