use clap::Parser;
use colored::*;

mod cli;
mod calendar;
mod utils;

use cli::{Cli, Commands};
use calendar::Calendar;
use utils::{normalize_day, capitalize_first};

fn main() {
    let cli = Cli::parse();
    let mut calendar = Calendar::load();

    match cli.command {
        Commands::Add { day, item } => {
            calendar.add_item(&day, &item);
            calendar.save();
            println!("{} '{}' {} {}", 
                "Added".green().bold(), 
                item.bright_white(), 
                "to".green(), 
                capitalize_first(&normalize_day(&day)).bright_cyan().bold()
            );
        }
        Commands::Show => {
            calendar.show();
        }
    }
}
