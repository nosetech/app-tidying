mod applescript;
mod cli;
mod config;
mod logger;

use clap::Parser;

use cli::{Cli, Commands};
use logger::LoggerConfig;

fn main() {
    let args = Cli::parse();

    let logger_config = LoggerConfig {
        debug_mode: args.verbose,
        notification_config: None,
    };

    logger::init(logger_config);

    log::info!("App Tidying started");
    log::debug!("Command: {:?}", args.command);

    match args.command {
        Commands::Load { path } => {
            if let Some(path) = path {
                log::info!("Loading layout from: {}", path.display());
            } else {
                log::info!("Loading layout from default configuration");
            }
        }
        Commands::Save { path, own } => {
            if let Some(path) = path {
                log::info!("Saving layout to: {}", path.display());
            } else {
                log::info!("Saving layout to default configuration");
            }
            if own {
                log::info!("Including terminal window where apptidying is running");
            }
        }
    }
}
