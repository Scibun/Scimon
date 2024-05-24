extern crate colored;
extern crate figlet_rs;

use colored::*;
use figlet_rs::FIGfont;
use indicatif::ProgressStyle;

use crate::{
    system::system::System,
    consts::global::Global,
    configs::settings::Settings,
};

pub struct UI;

impl UI {

    pub fn header() {
        if Settings::get("ui.show_header", "BOOLEAN") == true {
            let standard_font = FIGfont::standard().unwrap();

            if let Some(title) = standard_font.convert(Global::APP_NAME) {
                println!("{}", title.to_string().blue());
                println!("-------------------------------------------------------------------");
                println!("📜 Version: {}", Global::APP_VERSION.yellow());
                println!("🏠 Homepage: {} • {}", Global::APP_HOMEPAGE.blue(), Global::APP_AUTHOR.green());
                println!("⏰ Started in: {}", System::date_time().blue());
                println!("-------------------------------------------------------------------");
            }
        }
    }
 
    pub fn pb_template() -> ProgressStyle {
        ProgressStyle::with_template(Global::PB_STYLE).unwrap().progress_chars("█░")
    }

}
