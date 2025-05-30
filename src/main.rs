use clap::{Parser, Subcommand};
use colored::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use chrono::{DateTime, Utc};

#[derive(Parser)]
#[command(name = "calendar")]
#[command(about = "A simple CLI calendar tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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

#[derive(Serialize, Deserialize, Clone)]
struct Task {
    description: String,
    location: Option<String>,
}

impl Task {
    fn new(description: String, location: Option<String>) -> Self {
        Task { description, location }
    }
}

#[derive(Serialize, Deserialize, Default)]
struct Calendar {
    days: HashMap<String, Vec<Task>>,
}

impl Calendar {
    fn load() -> Self {
        let path = get_calendar_path();
        if path.exists() {
            let content = fs::read_to_string(&path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Calendar::default()
        }
    }

    fn save(&self) {
        let path = get_calendar_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).ok();
        }
        let content = serde_json::to_string_pretty(self).unwrap();
        fs::write(&path, content).ok();
    }

    fn add_item(&mut self, day: &str, item: &str, location: Option<String>) {
        let day = normalize_day(day);
        let task = Task::new(item.to_string(), location);
        self.days.entry(day).or_insert_with(Vec::new).push(task);
    }

    fn clear_day(&mut self, day: &str) {
        let day = normalize_day(day);
        self.days.insert(day, Vec::new());
    }

    fn show(&self) {
        let days_order = ["monday", "tuesday", "wednesday", "thursday", "friday", "saturday", "sunday"];
        // Custom mapping so Monday is green and Wednesday is red
let custom_colored_days = [
    ("monday", "green"),
    ("tuesday", "blue"),
    ("wednesday", "red"),
    ("thursday", "yellow"),
    ("friday", "magenta"),
    ("saturday", "cyan"),
    ("sunday", "white")
];
        
        println!("{}", "Weekly Calendar:".bold().bright_blue());
        println!("{}", "================".bright_blue());
        
        for day in days_order {
            let day_name = capitalize_first(day);
            let color = custom_colored_days.iter()
                .find(|(d, _)| *d == day)
                .map(|(_, c)| *c)
                .unwrap_or("white");
            
            println!();
            match color {
                "red" => println!("{}:", day_name.bright_red().bold()),
                "blue" => println!("{}:", day_name.bright_blue().bold()),
                "green" => println!("{}:", day_name.bright_green().bold()),
                "yellow" => println!("{}:", day_name.bright_yellow().bold()),
                "magenta" => println!("{}:", day_name.bright_magenta().bold()),
                "cyan" => println!("{}:", day_name.bright_cyan().bold()),
                _ => println!("{}:", day_name.bright_white().bold()),
            }
            
            if let Some(tasks) = self.days.get(day) {
                if tasks.is_empty() {
                    println!("  {}", "(no items)".dimmed());
                } else {
                    for task in tasks {
                        if let Some(location) = &task.location {
                            println!("  {} {} {}", "•".green(), task.description, format!("(at {})", location).dimmed());
                        } else {
                            println!("  {} {}", "•".green(), task.description);
                        }
                    }
                }
            } else {
                println!("  {}", "(no items)".dimmed());
            }
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
struct ArchivedWeek {
    timestamp: String, // ISO 8601 string
    week: HashMap<String, Vec<Task>>, // days -> tasks
}

fn get_calendar_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(".calendar");
    path.push("calendar.json");
    path
}

fn get_archive_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(".calendar");
    path.push("archive.json");
    path
}

fn archive_week(week: &Calendar) {
    let archive_path = get_archive_path();
    let mut archives: Vec<ArchivedWeek> = if archive_path.exists() {
        let content = fs::read_to_string(&archive_path).unwrap_or_default();
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        Vec::new()
    };
    let now: DateTime<Utc> = Utc::now();
    archives.push(ArchivedWeek {
        timestamp: now.to_rfc3339(),
        week: week.days.clone(),
    });
    if let Some(parent) = archive_path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let data = serde_json::to_string_pretty(&archives).unwrap();
    let _ = fs::write(&archive_path, data);
}

fn normalize_day(day: &str) -> String {
    let day = day.to_lowercase();
    
    // Handle shorthand day names
    match day.as_str() {
        "m" => "monday".to_string(),
        "t" => "tuesday".to_string(),
        "w" => "wednesday".to_string(),
        "th" => "thursday".to_string(),
        "f" => "friday".to_string(),
        "sa" | "s" => "saturday".to_string(),
        "su" => "sunday".to_string(),
        _ => day
    }
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

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
                item.bright_white(),
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
