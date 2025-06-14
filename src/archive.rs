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

pub fn archive_week(week: &Calendar) -> Vec<(String, Vec<Task>)> {
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
    
    // Collect recurring tasks to be preserved
    let mut recurring_tasks = Vec::new();
    for (day, tasks) in &week.days {
        let recurring = tasks.iter()
            .filter(|task| task.recurring)
            .cloned()
            .collect::<Vec<_>>();
        
        if !recurring.is_empty() {
            recurring_tasks.push((day.clone(), recurring));
        }
    }
    
    recurring_tasks
}