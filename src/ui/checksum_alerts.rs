extern crate colored;

use colored::*;

use crate::{
    system::system::System,
    cmd::checksum::Checksum,
    
    ui::{
        ui_base::UI,
        emojis::Emojis,
    },
};

pub struct ChecksumAlerts;

impl ChecksumAlerts {

    pub fn created(file: &str) {
        let current_datetime = System::date_time();

        UI::section_header("checksum");

        println!(
            "[{}] Hashes file created with successfully ({})", current_datetime.blue(), file.cyan()
        );
    }

    pub fn is_equal(line: &str) {
        if let Ok((hash, filename)) = Checksum::extract_hashes_and_filenames(line) {
            let current_datetime = System::date_time();

            println!(
                "[{}] -> The hash {} of {} is match. {}", 
                current_datetime.green(), 
                hash.yellow(), 
                filename.blue(), 
                Emojis::CHECKED
            );
        }
    }

    pub fn is_different(line: &str) {
        if let Ok((hash, filename)) = Checksum::extract_hashes_and_filenames(line) {
            let current_datetime = System::date_time();

            println!(
                "[{}] -> The hash {} of {} is not match. {}", 
                current_datetime.red(), 
                hash.yellow(), 
                filename.blue(), 
                Emojis::ERROR
            );
        }
    }

    pub fn check_content(is_error: bool) {
        let text;
        let emoji;

        let current_datetime = System::date_time();

        if is_error {
            emoji = Emojis::ERROR;
            text = "is not match".red();
        } else {
            emoji = Emojis::CHECKED;
            text = "is match".green();
        }

        println!(
            "[{}] -> The list {} and {} list {}. {}", 
            current_datetime.green(), 
            "local".cyan(),
            "remote".blue(),
            text, 
            emoji, 
        );
    }

    pub fn lines_total_is_different(local_lines: usize, remote_lines: usize) {
        let current_datetime = System::date_time();

        println!("[{}] -> The number of lines in the files is different (Lines: {} (local) of {} (referencies)). ❌",
            current_datetime.red(),
            local_lines, 
            remote_lines
        );
    }

}
