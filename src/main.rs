mod cli;
use cli::Args;

mod config;
use config::Config;

mod formatter;
use formatter::Formatter;

mod schedule_parser;

use ansi_term::Color::*;
use anyhow::Result;

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
        let default_template =
            "{stream_time}  {author_name}  {stream_url}  {stream_title}".to_owned();

        let template = args.format.to_owned().unwrap_or(default_template);
        let formatter = Formatter::new(&template);

        let lives = schedule_parser::get_schedule(&args, &cfg).await?;
        formatter.print(&lives);
    }

    Ok(())
}
