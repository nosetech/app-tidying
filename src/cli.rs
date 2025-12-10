use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "apptidying")]
#[command(about = "macOS application window layout management tool")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Enable verbose/debug output
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Restore window layout from a configuration file
    #[command(about = "Restore window layout")]
    Load {
        /// Path to the layout configuration file (defaults to ~/Library/Application Support/biz.nosetech.apptidying/settings.json)
        path: Option<PathBuf>,
    },
    /// Save current window layout to a configuration file
    #[command(about = "Save current window layout")]
    Save {
        /// Path to save the layout configuration file (defaults to ~/Library/Application Support/biz.nosetech.apptidying/settings.json)
        path: Option<PathBuf>,

        /// Include the terminal window where apptidying is running
        #[arg(long)]
        own: bool,
    },
}
