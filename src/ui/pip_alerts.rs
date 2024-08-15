extern crate colored;

use colored::*;

use crate::system::general::General;

pub struct PipAlerts;

impl PipAlerts {

    pub fn success() {
        let current_datetime = General::date_time();
        println!("{} Dependencies was installed successfully", current_datetime.green().bold());
    }

    pub fn error_install(e: &str) {
        let current_datetime = General::date_time();

        eprintln!(
            "{} Failed to install the dependencies: {}", 
            current_datetime.red().bold(), 
            e.red()
        );
    }

    pub fn error_check(package: &str) {
        let current_datetime = General::date_time();

        eprintln!(
            "{} Failed to check for package '{}'", 
            current_datetime.red().bold(),
            package.yellow(),
        );
    }

}
