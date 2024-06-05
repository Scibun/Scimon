extern crate colored;

use colored::*;

use crate::{
    ui::emojis::Emojis,
    system::system::System,
};

pub struct CompressAlerts;

impl CompressAlerts {

    pub fn added(file: &str, zip_file: &str) {
        let current_datetime = System::date_time();
    
        println!(
            "[{}] -> Added {} in: {}. {}", 
            current_datetime.green(), 
            file.blue(), 
            zip_file.cyan(),
            Emojis::ADD
        );
    }
  
    pub fn completed(zip_file: &str) {
        let current_datetime = System::date_time();
    
        println!(
            "[{}] -> All files in the folder have been compressed into {}. {}", 
            current_datetime.green(), 
            zip_file.blue(),
            Emojis::COMPRESS
        );
    }

}
