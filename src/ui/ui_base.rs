extern crate colored;
extern crate figlet_rs;

use colored::*;
use figlet_rs::FIGfont;

use crate::{
    utils::misc::Misc,
    configs::global::Global
};

pub struct PaimonUI;

impl PaimonUI {

    pub fn header() {
        let standard_font = FIGfont::standard().unwrap();

        if let Some(title) = standard_font.convert(Global::APP_NAME) {
            println!("{}", title.to_string().blue());
            println!("-------------------------------------------------------------------");
            println!("📜 Version: {}", Global::APP_VERSION.yellow());
            println!("🏠 Homepage: {} | {}", Global::APP_HOMEPAGE.blue(), Global::APP_AUTHOR.green());
            println!("⏰ Started in: {}", Misc::date_time().blue());
            println!("-------------------------------------------------------------------");
        }
    }

}
