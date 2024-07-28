extern crate chrono;

use std::path::PathBuf;
use once_cell::sync::Lazy;

use dirs_next::{
    config_dir,
    download_dir,
};

use crate::consts::global::Global;

pub struct Folders;

impl Folders {

    pub const APP_FOLDER: Lazy<PathBuf> = Lazy::new(|| {
        let mut path = config_dir().expect("No config directory");
        path.push(Global::APP_NAME);
        path
    });

    pub const DOWNLOAD_FOLDER: Lazy<PathBuf> = Lazy::new(|| {
        let mut path = download_dir().expect("No config directory");
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
        let mut path = Self::APP_FOLDER.clone();
        path.push("readme");
        path
    });
    
    pub const SCRIPTS_FOLDER: Lazy<PathBuf> = Lazy::new(|| {
        let mut path = Self::APP_FOLDER.clone();
        path.push("scripts");
        path
    });
    
    pub const PLUGINS_FOLDER: Lazy<PathBuf> = Lazy::new(|| {
        let mut path = Self::APP_FOLDER.clone();
        path.push("plugins");
        path
    });
    
    pub const SCRAPE_FOLDER: Lazy<PathBuf> = Lazy::new(|| {
        let mut path = Self::DOWNLOAD_FOLDER.clone();
        path.push("scrape");
        path
    });
    
}
