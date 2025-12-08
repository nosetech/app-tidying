mod cli;
mod config;
mod logger;

use cli::Cli;
use clap::Parser;

fn main() {
    let args = Cli::parse();

    logger::init();

    log::info!("App Tidying started");
    log::debug!("Command: {:?}", args);
}
