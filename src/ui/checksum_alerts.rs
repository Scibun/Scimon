extern crate colored;

use colored::*;

use crate::{
    system::{
        system::System,
        hashes::Hashes,
    },
    
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

    pub fn is_equal(line: &str) -> bool {
        if let Ok((hash, filename)) = Hashes::extract_hash_and_filename(line) {
            let current_datetime = System::date_time();

            println!(
                "[{}] -> The hash {} of {} is match. {}", 
                current_datetime.green(), 
                hash.yellow(), 
                filename.blue(), 
                Emojis::CHECKED
            );
        }

        false
    }

    pub fn is_different(line: &str) -> bool {
        if let Ok((hash, filename)) = Hashes::extract_hash_and_filename(line) {
            let current_datetime = System::date_time();

            println!(
                "[{}] -> The hash {} of {} is not match. {}", 
                current_datetime.red(), 
                hash.yellow(), 
                filename.blue(), 
                Emojis::ERROR
            );
        }

        true
    }

    pub fn lines_total_is_different(local_total_lines: usize, remote_total_lines: usize) {
        if local_total_lines != remote_total_lines {
            let current_datetime = System::date_time();

            println!("[{}] -> The number of lines in the files is different (Lines: {} (local) of {} (referencies)). {}",
                current_datetime.red(),
                local_total_lines, 
                remote_total_lines,
                Emojis::ERROR
            );
        }
    }

    pub fn lines_unmatch_file_deleted(filename: &str) {
        let current_datetime = System::date_time();

        println!("[{}] -> The file {} was deleted, because is hashes unmatched. {}",
            current_datetime.red(),
            filename.blue(),
            Emojis::FORBIDDEN
        );
    }

}
