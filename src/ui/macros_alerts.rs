extern crate colored;

use colored::*;

use crate::{
    ui::ui_base::UI,
    system::system::System,
};

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
        let current_datetime = System::date_time();

        UI::section_header("readme");

        println!(
            "[{}] README file rendered with successfully ({})", current_datetime.blue(), file.cyan()
        );

        println!("-----------------------------");
    }

    pub fn comments(line: &str) {
        let current_datetime = System::date_time();
        let line_without_macros = line.replace("!debug", "");

        UI::section_header("comment");

        println!(
            "[{}] {}", current_datetime.blue(), line_without_macros.yellow()
        );

        println!("-----------------------------");
    }

}
