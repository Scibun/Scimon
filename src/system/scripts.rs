extern crate chrono;

use regex::Regex;

use std::{
    error::Error,

    process::{
        Stdio,
        Command,
    },
};

use crate::{
    regexp::regex_core::CoreRegExp,
    ui::errors_commands_alerts::ErrorsCommandsAlerts,
};

pub struct Scripts;

impl Scripts {
    
    pub fn exec(line: &str, program: &str) -> Result<(), Box<dyn Error>> {
        let line_cleanned = Regex::new(
            CoreRegExp::CLEAN_LINE
        ).unwrap().replace_all(
            &line, ""
        ).to_string();

        let output = Command::new(&program)
            .arg(line_cleanned)
            .stdout(Stdio::piped())
            .output()?;
        
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("{}", stdout);
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            ErrorsCommandsAlerts::executing(&stderr);
        }

        Ok(())
    }

    pub fn read(line_trimmed: &str) -> Result<(), Box<dyn Error>> {
        if line_trimmed.len() >= 3 {
            if line_trimmed.ends_with(".py") {
                Self::exec(&line_trimmed, "python")?;
            } else if line_trimmed.ends_with(".js") {
                Self::exec(&line_trimmed, "node")?;
            } else {
                ErrorsCommandsAlerts::unsupported(&line_trimmed);
            }
        }

        Ok(())
    }

}
