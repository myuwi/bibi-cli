use clap::Parser;

const ABOUT: &str = "A simple Hololive Schedule CLI tool.
Shows current and upcoming streams by default.";

#[derive(Debug, Parser)]
#[clap(about = ABOUT, version )]
pub struct Args {
    #[clap(short, long, help = "Print a cute Bibi ascii art")]
    pub ascii: bool,

    #[clap(short, long, help = "Show streams that have ended")]
    pub ended: bool,

    #[clap(short, long, help = "Show streams that are currently live")]
    pub live: bool,

    #[clap(short, long, help = "Show streams that have not started yet")]
    pub upcoming: bool,
}

impl Args {
    pub fn new() -> Self {
        Self::parse()
    }
}
