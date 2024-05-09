use std::error::Error;

use crate::{
    utils::url::UrlMisc,
    ui::ui_alerts::PaimonUIAlerts
};

pub struct Lexico;

impl Lexico {

    pub fn handle_get_path(line: &str) -> String {
        if line.contains("!path") {
            line.replace(
                "!path", ""
            ).replace(
                "'", ""
            )
        } else {
            "".to_string()
        }
    }

    pub fn handle_check_macro_line(line: &str, word: &str) -> bool {
        let get_macro = format!("!{}", &word);
        line.contains(&get_macro.to_string())
    }

    pub fn handle_comments(line: &str, no_comments: bool) -> Result<(), Box<dyn Error>> {
        if !no_comments && line.contains("!debug") {
            PaimonUIAlerts::comments(line);
        }
    
        Ok(())
    }
    
    pub fn handle_ignore_macro_flag(line: &str, no_ignore: bool) -> Result<String, &'static str> {
        if !no_ignore && line.to_lowercase().contains("!ignore") {
            PaimonUIAlerts::ignore(line);
            return Err("Line contains the '!ignore' directive.");
        }
    
        Ok(
            UrlMisc::extract_url(line)
        )
    }

}
