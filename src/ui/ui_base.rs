extern crate colored;
extern crate figlet_rs;

use colored::*;
use figlet_rs::FIGfont;
use indicatif::ProgressStyle;

use crate::{
    ui::emojis::Emojis,
    utils::str::StrUtils,
    consts::global::Global,
    system::system::System,
    configs::settings::Settings,
};

pub struct UI;

impl UI {

    pub fn header() {
        if Settings::get("ui.show_header", "BOOLEAN") == true {
            let render_md_mode;
            let name = StrUtils::capitalize(Global::APP_NAME);
            let standard_font = FIGfont::standard().unwrap();

            if Settings::get("render_markdown.mode", "STRING") == Global::APP_NAME {
                render_md_mode = format!("{}", name.cyan());
            } else {
                render_md_mode = format!("{}", "Vanilla".cyan());
            }

            if let Some(title) = standard_font.convert(&name) {
                println!("{}", &title.to_string().bold().cyan());
                println!("-------------------------------------------------------------------");
                println!("{} Version: {}", Emojis::VERSION, Global::APP_VERSION.yellow());
                println!("{} Homepage: {} • {}", Emojis::HOME, Global::APP_HOMEPAGE.blue(), Global::APP_AUTHOR.green());
                println!("{} Started in: {}", Emojis::CLOCK, System::date_time().magenta());
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
        ProgressStyle::with_template(
            "[{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})"
        ).unwrap().progress_chars("█░")
    }

}
