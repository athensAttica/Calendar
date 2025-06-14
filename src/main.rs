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
        Commands::Add { day, item, location, recurring } => {
            let location_msg = if let Some(loc) = &location {
                format!(" at {}", loc)
            } else {
                String::new()
            };
            
            let recurring_msg = if recurring {
                " (recurring weekly)".bright_green()
            } else {
                "".normal()
            };
            
            calendar.add_item(&day, &item, location, recurring);
            calendar.save();
            
            println!("{} '{}{}'{} {} {}", 
                "Added".green().bold(), 
                item.bright_white(),
                location_msg.dimmed(),
                recurring_msg,
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
            let recurring_tasks = archive_week(&calendar);
            calendar.days.clear();
            
            // Re-add recurring tasks
            for (day, tasks) in recurring_tasks {
                for task in tasks {
                    calendar.days.entry(day.clone())
                        .or_insert_with(Vec::new)
                        .push(task);
                }
            }
            
            calendar.save();
            println!("{}", "Week archived and cleared successfully (recurring tasks preserved)".green().bold());
        }
    }
}
