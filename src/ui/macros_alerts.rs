extern crate colored;

use colored::*;

use crate::ui::ui_base::UI;

pub struct MacrosAlerts;

impl MacrosAlerts {

    pub fn ignore(line: &str) {
        let current_datetime = UI::date_time();
        let url = line.replace(" !ignore", "");

        eprintln!(
            "[{}] -> The url {} was ignored", current_datetime.green(), url.blue()
        );
    }

    pub fn comments(line: &str) {
        let comment_word = "Comment";
        let current_datetime = UI::date_time();
        let line_without_macros = line.replace("!debug", "");

        println!("---------- {} ----------", comment_word.yellow());

        println!(
            "[{}] {}", current_datetime.blue(), line_without_macros.yellow()
        );

        println!("-----------------------------");
    }

}
