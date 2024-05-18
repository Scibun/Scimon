extern crate colored;

use colored::*;

use crate::system::system::System;

pub struct MacrosAlerts;

impl MacrosAlerts {

    pub fn ignore(line: &str) {
        let current_datetime = System::date_time();
        let url = line.replace(" !ignore", "");

        println!(
            "[{}] -> The url {} was ignored", current_datetime.green(), url.blue()
        );
    }

    pub fn readme(file: &str) {
        let readme_word = "README";
        let current_datetime = System::date_time();

        println!("---------- {} -----------", readme_word.yellow());

        println!(
            "[{}] README file rendered with successfully ({})", current_datetime.blue(), file.cyan()
        );

        println!("-----------------------------");
    }

    pub fn comments(line: &str) {
        let comment_word = "Comment";
        let current_datetime = System::date_time();
        let line_without_macros = line.replace("!debug", "");

        println!("---------- {} ----------", comment_word.yellow());

        println!(
            "[{}] {}", current_datetime.blue(), line_without_macros.yellow()
        );

        println!("-----------------------------");
    }

}
