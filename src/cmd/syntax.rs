extern crate colored;

use colored::*;

use std::error::Error;

use crate::utils::misc::Misc;

pub struct Lexico;

impl Lexico {

    pub fn handle_comments(line: &str, no_comments: bool) -> Result<(), Box<dyn Error>> {
        if !no_comments && line.contains("!debug") {
            let comment_word: &str = "Comment"; 
            println!("---------- {} ----------", comment_word.yellow());
    
            eprintln!(
                "[{}] {}", Misc::date_time().blue(), line.replace(
                    "!debug", ""
                ).yellow()
            );
    
            println!("-----------------------------");
        }
    
        Ok(())
    }
    
    pub fn handle_ignore_macro_flag(line: &str, no_ignore: bool) -> Result<String, &'static str> {
        if !no_ignore && line.to_lowercase().contains("!ignore") {
            let url = line.replace(" !ignore", "");
            
            eprintln!(
                "[{}] -> The url {} was ignored", Misc::date_time().green(), url.blue()
            );
    
            return Err("Line contains the '!ignore' directive.");
        }
    
        Ok(
            line.replace(
                " !ignore", ""
            )
        )
    }

}
