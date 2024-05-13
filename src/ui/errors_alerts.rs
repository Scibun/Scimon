extern crate colored;

use colored::*;
use::std::error::Error;

use crate::ui::ui_base::UI;

pub struct ErrorsAlerts;

impl ErrorsAlerts {

    pub fn env(e: &str) {
        let current_datetime = UI::date_time();
        eprintln!("[{}] -> Failed to download the file: {}", current_datetime.blue(), e.red());
    }

    pub fn generic(e: &str) {
        let current_datetime = UI::date_time();

        eprintln!(
            "[{}] -> Error: {}", current_datetime.blue(), e.red()
        );
    }

    pub fn download(e: Box<dyn Error>, url: &str) {
        let e = e.to_string();
        let current_datetime = UI::date_time();

        eprintln!(
            "[{}] -> Error: {} (from {})", current_datetime.blue(), e.red(), url.cyan()
        );
    }

}
