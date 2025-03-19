use ansi_term::Color::{Purple, Yellow};
use anyhow::Result;
use clap::Parser;

mod cli;
mod config;
mod format;
mod schedule_parser;
mod types;

use cli::Args;
use config::Config;

const BIBI_ASCII: &str = "
  d8b, |        |        | ,d8b
 88888,|        |        |,88888
'88888.|        |        |.88888'
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
    let args = Args::parse();
    let cfg = Config::load(&args.config)?;

    if args.ascii {
        print_bibi();
    } else {
        let lives = schedule_parser::get_schedule(&args, &cfg).await?;

        let default_template =
            "{stream_time}  {author_name}  {stream_url}  {stream_title}".to_string();
        let format = args.format.unwrap_or(default_template);

        format::print(&format, &lives);
    }

    Ok(())
}
