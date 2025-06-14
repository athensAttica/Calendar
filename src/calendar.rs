use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use colored::*;
use crate::utils::{get_calendar_path, normalize_day, capitalize_first};

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub description: String,
    pub location: Option<String>,
    pub recurring: bool,
}

impl Task {
    pub fn new(description: String, location: Option<String>, recurring: bool) -> Self {
        Task { description, location, recurring }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Calendar {
    pub days: HashMap<String, Vec<Task>>,
}

impl Calendar {
    pub fn load() -> Self {
        let path = get_calendar_path();
        if path.exists() {
            let content = fs::read_to_string(&path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Calendar::default()
        }
    }

    pub fn save(&self) {
        let path = get_calendar_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).ok();
        }
        let content = serde_json::to_string_pretty(self).unwrap();
        fs::write(&path, content).ok();
    }

    pub fn add_item(&mut self, day: &str, item: &str, location: Option<String>, recurring: bool) {
        let day = normalize_day(day);
        let task = Task::new(item.to_string(), location, recurring);
        self.days.entry(day).or_insert_with(Vec::new).push(task);
    }

    pub fn clear_day(&mut self, day: &str) {
        let day = normalize_day(day);
        self.days.insert(day, Vec::new());
    }

    pub fn show(&self) {
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
                        let recurring_indicator = if task.recurring { " ↻".bright_green() } else { "".normal() };
                        if let Some(location) = &task.location {
                            println!("  {} {}{} {}", "•".green(), task.description.bright_yellow(), recurring_indicator, format!("(at {})", location).dimmed());
                        } else {
                            println!("  {} {}{}", "•".green(), task.description.bright_yellow(), recurring_indicator);
                        }
                    }
                }
            } else {
                println!("  {}", "(no items)".dimmed());
            }
        }
    }
}