use log;
use simple_logger::SimpleLogger;
use clap::{Parser};
use tricksys_find::app::args::Args;

fn main() {
    // Logger
    SimpleLogger::new().init().unwrap();
    log::trace!("App started");

    // Parse args
    let _args = Args::parse();

}
