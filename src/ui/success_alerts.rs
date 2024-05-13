extern crate colored;

use colored::*;

use crate::utils::{
    url::UrlMisc,
    system::System,
};

pub struct SuccessAlerts;

impl SuccessAlerts {

    pub fn env() {
        let current_datetime = System::date_time();
        println!("[{}] -> Downloaded env file", current_datetime.blue());
    }

    pub fn download(file: &str, url: &str) {
        let domain = UrlMisc::get_domain(url);
        let current_datetime = System::date_time();
    
        println!(
            "[{}] -> Downloaded file name: {} (from: {})", current_datetime.green(), file.blue(), domain.cyan(),
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
