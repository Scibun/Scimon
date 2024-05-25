extern crate colored;
extern crate figlet_rs;

use colored::*;
use figlet_rs::FIGfont;
use indicatif::ProgressStyle;

use crate::{
    ui::emojis::Emojis,
    system::system::System,
    configs::settings::Settings,
    
    consts::{
        global::Global,
        prime_down::PrimeDownEnv,
    },
};

pub struct UI;

impl UI {

    pub fn header() {
        if Settings::get("ui.show_header", "BOOLEAN") == true {
            let render_md_mode;
            let standard_font = FIGfont::standard().unwrap();

            if Settings::get("render_markdown.mode", "STRING") == "paimon" {
                render_md_mode = format!("{} ({})", "Paimon".cyan(), PrimeDownEnv::README_PAIMON_MODE_DOC.blue());
            } else {
                render_md_mode = format!("{}", "Vanilla".cyan());
            }

            if let Some(title) = standard_font.convert(Global::APP_NAME) {
                println!("{}", title.to_string().bold().cyan());
                println!("-------------------------------------------------------------------");
                println!("{} Version: {}", Emojis::VERSION, Global::APP_VERSION.yellow());
                println!("{} Homepage: {} • {}", Emojis::HOME, Global::APP_HOMEPAGE.blue(), Global::APP_AUTHOR.green());
                println!("{} Started in: {}", Emojis::CLOCK, System::date_time().blue());
                println!("{} Render mode: {}", Emojis::TOOLS, render_md_mode);
                println!("-------------------------------------------------------------------");
            }
        }
    }
    
    pub fn section_header(text: &str) {
        let text = text.to_uppercase();
        println!("---------- {} -----------", text.yellow());
    }

    pub fn pb_template() -> ProgressStyle {
        ProgressStyle::with_template(Global::PB_STYLE).unwrap().progress_chars("█░")
    }

}
