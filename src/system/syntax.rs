use regex::Regex;
use std::error::Error;

use crate::{
    utils::url::UrlMisc,
    ui::macros_alerts::MacrosAlerts,
    regex::regex_macros::MacrosRegExp,
};

pub struct Macros;

impl Macros {

    pub fn remove_macros(input: &str) -> String {
        let re = Regex::new(MacrosRegExp::GET_MACROS).unwrap();
        re.replace_all(input, "").to_string()
    }

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
    
    pub fn remove_readme_macros_html(markdown_html: String) -> String {
        markdown_html.replace(
            "<p>!readme</p>\n", ""
        ).replace(
            "<p>!end_readme</p>\n", ""
        )
    }

    pub fn handle_comments(line: &str, no_comments: bool) -> Result<(), Box<dyn Error>> {
        if !no_comments && line.contains("!debug") {
            MacrosAlerts::comments(line);
        }
    
        Ok(())
    }
    
    pub fn handle_ignore_macro_flag(line: &str, no_ignore: bool) -> Result<String, &'static str> {
        if !no_ignore && line.to_lowercase().contains("!ignore") {
            MacrosAlerts::ignore(line);
            return Err("Line contains the '!ignore' directive.");
        }
    
        Ok(
            UrlMisc::extract_url(line)
        )
    }

}
