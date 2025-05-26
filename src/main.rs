use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

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
    },
    /// Show all items for the week
    Show,
}

#[derive(Serialize, Deserialize, Default)]
struct Calendar {
    days: HashMap<String, Vec<String>>,
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

    fn add_item(&mut self, day: &str, item: &str) {
        let day = normalize_day(day);
        self.days.entry(day).or_insert_with(Vec::new).push(item.to_string());
    }

    fn show(&self) {
        let days_order = ["monday", "tuesday", "wednesday", "thursday", "friday", "saturday", "sunday"];
        
        println!("Weekly Calendar:");
        println!("================");
        
        for day in days_order {
            println!("\n{}:", capitalize_first(day));
            if let Some(items) = self.days.get(day) {
                if items.is_empty() {
                    println!("  (no items)");
                } else {
                    for item in items {
                        println!("  - {}", item);
                    }
                }
            } else {
                println!("  (no items)");
            }
        }
    }
}

fn get_calendar_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(".calendar");
    path.push("calendar.json");
    path
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
        Commands::Add { day, item } => {
            calendar.add_item(&day, &item);
            calendar.save();
            println!("Added '{}' to {}", item, capitalize_first(&normalize_day(&day)));
        }
        Commands::Show => {
            calendar.show();
        }
    }
}
