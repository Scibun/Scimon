extern crate colored;

use colored::*;

use crate::system::general::General;

pub struct CompressAlerts;

impl CompressAlerts {

    pub fn added(file: &str, zip_file: &str) {
        let current_datetime = General::date_time();
    
        println!(
            "{} Added {} in: {}", 
            current_datetime.green().bold(), 
            file.blue(), 
            zip_file.cyan(),
        );
    }
  
    pub fn completed(zip_file: &str) {
        let current_datetime = General::date_time();
    
        println!(
            "{} All files in the folder have been compressed into {}.", 
            current_datetime.green().bold(), 
            zip_file.blue(),
        );
    }

}
