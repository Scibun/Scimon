extern crate colored;

use colored::*;

use crate::{
    ui::emojis::Emojis,
    utils::domains::Domains,
    system::general::General,
};

pub struct SuccessAlerts;

impl SuccessAlerts {

    pub fn env() {
        let current_datetime = General::date_time();
        println!("{} Downloaded env file", current_datetime.blue().bold());
    }

    pub fn download(file: &str, url: &str, password: bool, hash: &str) {
        let mut encrypted_emoji = "";

        let domain = Domains::get(url);
        let current_datetime = General::date_time();
    
        if password {
            encrypted_emoji = Emojis::LOCKED;
        }
    
        println!(
            "{} Downloaded: {} ({} • {}) {}", 
            current_datetime.green().bold(), 
            file.blue(), 
            domain.cyan(), 
            hash.yellow(),
            encrypted_emoji
        );
    }
  
    pub fn download_and_generated_pdf(file: &str, url: &str, hash: &str) {
        let domain = Domains::get(url);
        let current_datetime = General::date_time();
    
        println!(
            "{} Downloaded and generated pdf: {} ({} • {})", 
            current_datetime.green().bold(), 
            file.blue(), 
            domain.cyan(),
            hash.yellow()
        );
    }

}
