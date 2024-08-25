extern crate colored;

use colored::*;

use crate::system::general::General;

pub struct ChecksumAlerts;

impl ChecksumAlerts {

    pub fn hash(file: &str, hash: &str) {
        println!(
            "{}: {}", 
            file.blue(), 
            hash, 
        );
    }

    pub fn checksum_file(file: &str) {
        let current_datetime = General::date_time();

        println!(
            "{} Checksum file created with successfully ({})", 
            current_datetime.blue().bold(), 
            file.cyan()
        );
    }

}
