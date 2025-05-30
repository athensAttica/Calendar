use std::path::PathBuf;

pub fn get_calendar_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(".calendar");
    path.push("calendar.json");
    path
}

pub fn normalize_day(day: &str) -> String {
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

pub fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}