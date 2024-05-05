extern crate chrono;

use regex::Regex;
use chrono::Local;

use crate::configs::regex::RegexRules;

pub struct Misc;

impl Misc {

    pub fn extract_url(line: &str) -> String {
        let re = Regex::new(RegexRules::EXTRACT_URL).unwrap();

        if let Some(capture) = re.find(line) {
            capture.as_str().to_string()
        } else {
            String::new()
        }
    }

    pub fn open_url(line: &str) {
        let get_url = Self::extract_url(line);
        let _ = webbrowser::open(&get_url);
    }

    pub fn date_time() -> String {
        let local_time = Local::now();
    
        let date_formated = local_time.format("%Y-%m-%d").to_string();
        let hour_formated = local_time.format("%H:%M:%S").to_string();
    
        format!("{} {}", date_formated, hour_formated)
    }
    
    pub fn check_format(input: &str) -> bool {
        let parts: Vec<&str> = input.split('/').collect();
        if parts.len() == 2 && input.starts_with('@') && !parts[1].is_empty() { return true; }
        false
    }
    
    pub fn remove_initial_character(text: &str, character: char) -> String {
        if let Some(rest) = text.strip_prefix(character) {
            return String::from(rest);
        }
        
        return String::from(text);
    }
    
}
