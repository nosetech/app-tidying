mod cli;
mod config;
mod logger;

use clap::Parser;

use cli::Cli;

fn main() {
    let args = Cli::parse();

    logger::init();

    log::info!("App Tidying started");
    log::debug!("Command: {:?}", args);
}
