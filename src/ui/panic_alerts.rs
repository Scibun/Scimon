extern crate colored;

use colored::*;
use::std::process::exit;

use crate::ui::ui_base::UI;

pub struct PanicAlerts;

impl PanicAlerts {

    fn message(message: &str) {
        eprintln!(
            "{}: {}",
            "Panic Error".bold().red(),
            message.bold()
        );

        exit(1);
    }

    pub fn downloads_block() {
        UI::section_header("block required not found: downloads", "error");
        Self::message("'downloads' block not found in file.");
    }

    pub fn path_var() {
        UI::section_header("var required not found: path", "error");
        Self::message("'path' variable not found in file.");
    }

    pub fn compress_level() {
        UI::section_header("compress level invalid", "error");
        Self::message("The compresss level set is invalid.");
    }

}
