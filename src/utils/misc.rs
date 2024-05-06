extern crate chrono;

use chrono::Local;

pub struct Misc;

impl Misc {

    pub fn date_time() -> String {
        let local_time = Local::now();
    
        let date_formated = local_time.format("%Y-%m-%d").to_string();
        let hour_formated = local_time.format("%H:%M:%S").to_string();
    
        format!("{} {}", date_formated, hour_formated)
    }
    
    pub fn check_is_user(input: &str) -> bool {
        let parts: Vec<&str> = input.split('/').collect();
        if parts.len() == 2 && input.starts_with('@') && !parts[1].is_empty() { return true }
        false
    }
    
    pub fn remove_initial_character(text: &str, character: char) -> String {
        if let Some(rest) = text.strip_prefix(character) {
            return String::from(rest);
        }
        
        return String::from(text);
    }
    
}
