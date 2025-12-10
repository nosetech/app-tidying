mod cli;
mod config;
mod logger;

use clap::Parser;

use cli::Cli;
use logger::LoggerConfig;

fn main() {
    let args = Cli::parse();

    let logger_config = LoggerConfig {
        debug_mode: args.verbose,
        notification_config: None,
    };

    logger::init(logger_config);

    log::info!("App Tidying started");
    log::debug!("Command: {:?}", args);
}
