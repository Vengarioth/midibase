use structopt::*;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(about = "Lists the available midi devices")]
    List,

    #[structopt(about = "Runs midibase")]
    Run {

        #[structopt(name = "CONFIG", short = "c", long = "config", default_value = "./config.json", help = "the full path to a configuration file")]
        config: PathBuf,

        #[structopt(name = "DEVICE", short = "d", long = "device", default_value = "0", help = "the index of the midi device to pick (use `midibase list` to list all devices)")]
        device: usize,

        #[structopt(name = "URL", short = "u", long = "url", default_value = "ws://localhost:4444", help = "the url to reach the obs-websocket plugin")]
        url: String,

        #[structopt(name = "POLL", short = "p", long = "poll", default_value = "250", help = "the websocket polling interval in milliseconds")]
        poll: u64,

        #[structopt(name = "VERBOSE", short = "v", long = "verbose", about = "gives verbose feedback")]
        verbose: bool,
    },
}

impl Command {
    pub fn parse() -> Self {
        Self::from_args()
    }
}
