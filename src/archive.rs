use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use chrono::{DateTime, Utc};
use crate::calendar::{Calendar, Task};
use crate::utils::get_archive_path;

#[derive(Serialize, Deserialize, Default)]
pub struct ArchivedWeek {
    pub timestamp: String, // ISO 8601 string
    pub week: HashMap<String, Vec<Task>>, // days -> tasks
}

pub fn archive_week(calendar: &Calendar) -> Vec<(String, Task)> {
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
        week: calendar.days.clone(),
    });
    if let Some(parent) = archive_path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let data = serde_json::to_string_pretty(&archives).unwrap();
    let _ = fs::write(&archive_path, data);
    
    // Collect recurring tasks to keep for next week
    let mut recurring_tasks: Vec<(String, Task)> = Vec::new();
    for (day, tasks) in &calendar.days {
        for task in tasks {
            if task.recurring {
                recurring_tasks.push((day.clone(), task.clone()));
            }
        }
    }
    
    recurring_tasks
}