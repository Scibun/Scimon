extern crate colored;

use colored::*;

use crate::{
    utils::url::UrlMisc,
    system::system::System,
};

pub struct SuccessAlerts;

impl SuccessAlerts {

    pub fn env() {
        let current_datetime = System::date_time();
        println!("[{}] -> Downloaded env file", current_datetime.blue());
    }

    pub fn download(file: &str, url: &str, password: bool) {
        let mut encrypted_emoji = "";

        let domain = UrlMisc::get_domain(url);
        let current_datetime = System::date_time();
    
        if password {
            encrypted_emoji = "🔒";
        }
    
        println!(
            "[{}] -> Downloaded file name: {} (from: {}) {}", 
            current_datetime.green(), 
            file.blue(), 
            domain.cyan(), 
            encrypted_emoji
        );
    }
  
    pub fn download_and_generated_pdf(file: &str, url: &str) {
        let domain = UrlMisc::get_domain(url);
        let current_datetime = System::date_time();
    
        println!(
            "[{}] -> Downloaded and generated pdf: {} (from: {})", current_datetime.green(), file.blue(), domain.cyan(),
        );
    }

}
