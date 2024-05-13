extern crate chrono;
extern crate colored;
extern crate figlet_rs;

use colored::*;
use chrono::Local;
use figlet_rs::FIGfont;
use indicatif::ProgressStyle;

use crate::configs::global::Global;

pub struct UI;

impl UI {

    pub const PB_STYLE: &'static str = "[{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})";

    pub fn header() {
        let standard_font = FIGfont::standard().unwrap();

        if let Some(title) = standard_font.convert(Global::APP_NAME) {
            println!("{}", title.to_string().blue());
            println!("-------------------------------------------------------------------");
            println!("📜 Version: {}", Global::APP_VERSION.yellow());
            println!("🏠 Homepage: {} | {}", Global::APP_HOMEPAGE.blue(), Global::APP_AUTHOR.green());
            println!("⏰ Started in: {}", Self::date_time().blue());
            println!("-------------------------------------------------------------------");
        }
    }
 
    pub fn date_time() -> String {
        let local_time = Local::now();
    
        let date_formated = local_time.format("%Y-%m-%d").to_string();
        let hour_formated = local_time.format("%H:%M:%S").to_string();
    
        format!("{} {}", date_formated, hour_formated)
    }
   
    pub fn pb_template() -> ProgressStyle {
        ProgressStyle::with_template(UI::PB_STYLE).unwrap().progress_chars("█░")
    }

}
