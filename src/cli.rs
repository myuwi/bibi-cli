use clap::Parser;
use std::path::PathBuf;

const ABOUT: &str = "A simple Hololive Schedule CLI tool.
Shows current and upcoming streams if no flags are provided.";

#[derive(Debug, Parser)]
#[command(about = ABOUT, version )]
pub struct Args {
    #[arg(short, long, help = "Show all streams")]
    pub all: bool,

    #[arg(long, help = "Print a cute Bibi ascii art")]
    pub ascii: bool,

    #[arg(short, long, help = "Config path", value_name = "PATH")]
    pub config: Option<PathBuf>,

    #[arg(short, long, help = "Output format", value_name = "FORMAT")]
    pub format: Option<String>,

    #[arg(
        short,
        long,
        help = "Show streams that are currently live",
        conflicts_with = "all"
    )]
    pub live: bool,

    #[arg(
        short,
        long,
        help = "Show streams that have ended",
        conflicts_with = "all"
    )]
    pub ended: bool,

    #[arg(
        short,
        long,
        help = "Show streams that have not started yet",
        conflicts_with = "all"
    )]
    pub upcoming: bool,
}

impl Args {
    pub fn new() -> Self {
        Self::parse()
    }
}
