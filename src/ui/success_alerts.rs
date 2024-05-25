extern crate colored;

use colored::*;

use crate::{
    ui::emojis::Emojis,
    system::system::System,
    utils::domains::Domains,
};

pub struct SuccessAlerts;

impl SuccessAlerts {

    pub fn env() {
        let current_datetime = System::date_time();
        println!("[{}] -> Downloaded env file", current_datetime.blue());
    }

    pub fn download(file: &str, url: &str, password: bool, hash: &str) {
        let mut encrypted_emoji = "";

        let domain = Domains::get(url);
        let current_datetime = System::date_time();
    
        if password {
            encrypted_emoji = Emojis::LOCKED;
        }
    
        println!(
            "[{}] -> Downloaded: {} (from: {} • sha256: {}) {}", 
            current_datetime.green(), 
            file.blue(), 
            domain.cyan(), 
            hash.yellow(),
            encrypted_emoji
        );
    }
  
    pub fn download_and_generated_pdf(file: &str, url: &str, hash: &str) {
        let domain = Domains::get(url);
        let current_datetime = System::date_time();
    
        println!(
            "[{}] -> Downloaded and generated pdf: {} (from: {} • sha256: {})", 
            current_datetime.green(), 
            file.blue(), 
            domain.cyan(),
            hash.yellow()
        );
    }

}
