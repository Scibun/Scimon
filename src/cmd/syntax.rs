extern crate colored;

use colored::*;

use std::error::Error;

pub fn handle_comments(line: &str) -> Result<(), Box<dyn Error>> {
    if line.contains("!debug") {
        eprintln!(
            "{}", line.replace(
                "!debug", ""
            ).yellow()
        );
    }

    Ok(())
}

pub fn handle_ignore_macro_flag(line: &str, no_ignore: bool) -> Result<String, &'static str> {
    if !no_ignore && line.to_lowercase().contains("!ignore") {
        let url = line.replace(" !ignore", "");
        
        eprintln!(
            "-> The url {} was ignored", url.blue()
        );

        return Ok("Line contains the '!ignore' directive.".to_owned());
    }

    Ok(
        line.replace(
            " !ignore", ""
        )
    )
}
