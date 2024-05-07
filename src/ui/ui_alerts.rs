extern crate colored;

use colored::*;

use::std::error::Error;

use crate::utils::misc::Misc;

pub struct PaimonUIAlerts;

impl PaimonUIAlerts {

    pub fn success_env() {
        let current_datetime = Misc::date_time();
        println!("[{}] -> Downloaded env file", current_datetime.blue());
    }

    pub fn error_env(e: &str) {
        let current_datetime = Misc::date_time();
        eprintln!("[{}] -> Failed to download the file: {}", current_datetime.blue(), e.red());
    }

    pub fn ignore(line: &str) {
        let current_datetime = Misc::date_time();
        let url = line.replace(" !ignore", "");

        eprintln!(
            "[{}] -> The url {} was ignored", current_datetime.green(), url.blue()
        );
    }

    pub fn comments(line: &str) {
        let comment_word = "Comment";
        let current_datetime = Misc::date_time();
        let line_without_macros = line.replace("!debug", "");

        println!("---------- {} ----------", comment_word.yellow());

        println!(
            "[{}] {}", current_datetime.blue(), line_without_macros.yellow()
        );

        println!("-----------------------------");
    }

    pub fn success_download(file: &str) {
        let current_datetime = Misc::date_time();

        println!(
            "[{}] -> Downloaded file name: {}", current_datetime.blue(), file.green()
        );
    }

    pub fn error_download(e: Box<dyn Error>) {
        let e = e.to_string();
        let current_datetime = Misc::date_time();

        eprintln!(
            "[{}] -> Error: downloading name: {}", current_datetime.blue(), e.red()
        );
    }

    pub fn generic_error(e: &str) {
        let current_datetime = Misc::date_time();

        eprintln!(
            "[{}] -> Error: {}", current_datetime.blue(), e.red()
        );
    }

    pub fn error_kindle(error: &str) {
        let current_datetime = Misc::date_time();

        eprintln!(
            "[{}] -> Could not send Kindle: {}", current_datetime.blue(), error.green()
        );
    }

    pub fn success_kindle(file: &str) {
        let current_datetime = Misc::date_time();

        println!(
            "[{}] -> Document sent to the Kindle, file: {}", current_datetime.blue(), file.green()
        );
    }

}
