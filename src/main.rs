use calendar_app::cli::{Cli, Commands};
use calendar_app::calendar::Calendar;
use calendar_app::archive::archive_week;
use calendar_app::utils::{normalize_day, capitalize_first};
use clap::Parser;
use colored::*;

fn main() {
    let cli = Cli::parse();
    let mut calendar = Calendar::load();

    match cli.command {
        Commands::Add { day, item, location } => {
            let location_msg = if let Some(loc) = &location {
                format!(" at {}", loc)
            } else {
                String::new()
            };
            
            calendar.add_item(&day, &item, location);
            calendar.save();
            
            println!("{} '{}{}' {} {}", 
                "Added".green().bold(), 
                item.bright_yellow(), // Changed to bright_yellow for user-added items
                location_msg.dimmed(),
                "to".green(), 
                capitalize_first(&normalize_day(&day)).bright_cyan().bold()
            );
        }
        Commands::Show => {
            calendar.show();
        }
        Commands::Clear { day } => {
            calendar.clear_day(&day);
            calendar.save();
            println!("{} {} {}", 
                "Cleared all items from".yellow().bold(),
                capitalize_first(&normalize_day(&day)).bright_cyan().bold(),
                "successfully".yellow().bold()
            );
        }
        Commands::ArchiveWeek => {
            archive_week(&calendar);
            calendar.days.clear();
            calendar.save();
            println!("{}", "Week archived and cleared successfully".green().bold());
        }
    }
}
