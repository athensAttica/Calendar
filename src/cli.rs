use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "calendar")]
#[command(about = "A simple CLI calendar tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add an item to a specific day
    Add {
        /// Day of the week
        #[arg(short, long)]
        day: String,
        /// Item to add
        #[arg(short, long)]
        item: String,
        /// Location where the task will happen (optional)
        #[arg(short, long)]
        location: Option<String>,
    },
    /// Show all items for the week
    Show,
    /// Clear all items from a specific day
    Clear {
        /// Day of the week
        #[arg(short, long)]
        day: String,
    },
    /// Archive the current week and clear all items
    ArchiveWeek,
}