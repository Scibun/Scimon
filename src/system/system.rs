extern crate chrono;

use chrono::Local;
use once_cell::sync::Lazy;
use dirs_next::config_dir;

use std::path::PathBuf;

use crate::consts::global::Global;

pub struct System;

impl System {

    pub const APP_FOLDER: Lazy<PathBuf> = Lazy::new(|| {
        let mut path = config_dir().expect("No config directory");
        path.push(Global::APP_NAME);
        path
    });
    
    pub const SETTINGS_FILE: Lazy<PathBuf> = Lazy::new(|| {
        let mut path = config_dir().expect("No config directory");
        path.push(Global::APP_NAME);
        path.push(
            format!("{}.yml", Global::APP_NAME)
        );

        path
    });
    
    pub const README_FOLDER: Lazy<PathBuf> = Lazy::new(|| {
        let mut path = System::APP_FOLDER.clone();
        path.push("readme");
        path
    });
    
    pub const SCRIPTS_FOLDER: Lazy<PathBuf> = Lazy::new(|| {
        let mut path = System::APP_FOLDER.clone();
        path.push("scripts");
        path
    });

    pub fn date_time() -> String {
        let local_time = Local::now();
    
        let date_formated = local_time.format("%Y-%m-%d").to_string();
        let hour_formated = local_time.format("%H:%M:%S").to_string();
    
        format!("{} {}", date_formated, hour_formated)
    }

}
