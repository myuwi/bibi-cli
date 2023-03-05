mod channels;

mod cli;
use cli::Args;

mod config;
use config::*;

mod schedule_parser;
use schedule_parser::{LiveStream, LiveStreamStatus};

use ansi_term::Color::*;
use ansi_term::Style;
use anyhow::Result;
use atty::Stream;
use chrono::prelude::*;
use unicode_width::UnicodeWidthStr;

fn print_lives(lives: &[LiveStream]) {
    // Get the width of the widest channel name
    let max_name_width = lives
        .iter()
        .fold(0, |acc, live| std::cmp::max(live.author_name.width(), acc));

    let use_color = atty::is(Stream::Stdout);

    for live in lives.iter() {
        let name_width = live.author_name.width();

        let white_space = " ".repeat(max_name_width - name_width);

        let local_time: DateTime<Local> = DateTime::from(live.time);

        // Default format: '{time}  {author_name}  {stream_url}  {stream_title}'
        let formatted = format!(
            "{}  {}{}  https://youtu.be/{}  {}",
            local_time.format("%H:%M"),
            live.author_name,
            white_space,
            live.id,
            live.title
        );

        let style = if use_color {
            match live.status {
                LiveStreamStatus::Ended => Fixed(8).normal(),
                LiveStreamStatus::Live => Purple.normal(),
                LiveStreamStatus::Upcoming => Style::default(),
            }
        } else {
            Style::default()
        };

        println!("{}", style.paint(formatted));
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
    let colors = [Purple.bold(), Yellow.bold(), Purple.bold(), Yellow.bold()];
    // Split Bibi ascii into lines while keeping line breaks
    for line in BIBI_ASCII.split_inclusive('\n') {
        // Split the lines on "|" and color the splits
        for (text, color) in line.split('|').zip(colors.iter().copied()) {
            print!("{}", color.paint(text));
        }
    }
    println!();
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::new();
    let cfg = Config::new(&args.config)?;

    if args.ascii {
        print_bibi();
    } else {
        let lives = schedule_parser::get_schedule(&args, &cfg).await?;
        print_lives(&lives);
    }

    Ok(())
}
