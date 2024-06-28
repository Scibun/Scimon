extern crate colored;

use colored::*;

use crate::system::general::General;

pub struct ErrorsCommandsAlerts;

impl ErrorsCommandsAlerts {

    pub fn executing(stderr: &str) {
        let current_datetime = General::date_time();

        eprintln!(
            "{} -> Error executing script: {}", 
            current_datetime.blue().bold(), 
            stderr.cyan()
        );
    }

    pub fn unsupported(script: &str) {
        let current_datetime = General::date_time();

        eprintln!(
            "{} -> Unsupported script: {}", 
            current_datetime.blue().bold(), 
            script.cyan()
        );
    }

}
