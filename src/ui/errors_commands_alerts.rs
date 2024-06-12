extern crate colored;

use colored::*;

use crate::system::system::System;

pub struct ErrorsCommandsAlerts;

impl ErrorsCommandsAlerts {

    pub fn executing(stderr: &str) {
        let current_datetime = System::date_time();

        eprintln!(
            "[{}] -> Error executing script: {}", current_datetime.blue(), stderr.cyan()
        );
    }

    pub fn unsupported(script: &str) {
        let current_datetime = System::date_time();

        eprintln!(
            "[{}] -> Unsupported script: {}", current_datetime.blue(), script.cyan()
        );
    }

}
