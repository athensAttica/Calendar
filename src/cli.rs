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
    },
    /// Show all items for the week
    Show,
}