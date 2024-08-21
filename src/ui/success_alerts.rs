extern crate colored;

use colored::*;

use crate::{
    utils::domain::Domain,
    system::general::General,
};

pub struct SuccessAlerts;

impl SuccessAlerts {

    pub fn env() {
        let current_datetime = General::date_time();
        println!("{} Downloaded env file", current_datetime.green().bold());
    }

    pub fn download(file: &str, url: &str, password: bool, hash: &str) {
        let mut encrypted_emoji = "";

        let domain = Domain::new(url).get();
        let current_datetime = General::date_time();
    
        if password {
            encrypted_emoji = "🔒";
        }
    
        println!(
            "{} Downloaded {} ({} • {}) {}", 
            current_datetime.green().bold(), 
            file.blue(), 
            domain.cyan(), 
            hash.yellow(),
            encrypted_emoji
        );
    }

    pub fn qrcode(file: &str) {
        let current_datetime = General::date_time();
    
        println!(
            "{} QR Code generated and saved in {}", 
            current_datetime.green().bold(), 
            file.blue(), 
        );
    }
  
    pub fn download_and_generated_pdf(file: &str, url: &str, hash: &str) {
        let domain = Domain::new(url).get();
        let current_datetime = General::date_time();
    
        println!(
            "{} Downloaded and generated pdf {} ({} • {})", 
            current_datetime.green().bold(), 
            file.blue(), 
            domain.cyan(),
            hash.yellow()
        );
    }
  
    pub fn cover_generated(file: &str) {
        let current_datetime = General::date_time();
    
        println!(
            "{} Cover saved in {}", 
            current_datetime.green().bold(), 
            file.blue(),
        );
    }

}
