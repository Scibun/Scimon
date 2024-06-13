extern crate chrono;

use once_cell::sync::Lazy;
use dirs_next::config_dir;

use std::path::PathBuf;

use crate::consts::global::Global;

pub struct Folders;

impl Folders {

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
        let mut path = Folders::APP_FOLDER.clone();
        path.push("readme");
        path
    });
    
    pub const SCRIPTS_FOLDER: Lazy<PathBuf> = Lazy::new(|| {
        let mut path = Folders::APP_FOLDER.clone();
        path.push("scripts");
        path
    });
    
}
