use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "App Tidying")]
#[command(about = "macOS application window layout management tool", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Save current window layout
    Save {
        /// Layout name
        name: String,
    },
    /// Restore window layout
    Restore {
        /// Layout name
        name: String,
    },
    /// List saved layouts
    List,
}
