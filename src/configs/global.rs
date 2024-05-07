use std::path::PathBuf;
use once_cell::sync::Lazy;
use dirs_next::config_dir;

pub struct Global;

impl Global {

    pub const APP_NAME: &'static str = "Paimon";
    pub const APP_VERSION: &'static str = "0.0.1";
    pub const APP_AUTHOR: &'static str = "@Ravenlib";
    pub const APP_HOMEPAGE: &'static str = "https://github.com/Ravenlib/Paimon";

    pub const README_TEMPLATE_FILE: &'static str = "templates/readme.html";
    
    pub const APP_FOLDER: Lazy<PathBuf> = Lazy::new(|| {
        let mut path = config_dir().expect("No config directory");
        path.push(Self::APP_NAME);
        path
    });

    pub const README_FOLDER: Lazy<PathBuf> = Lazy::new(|| {
        let mut path = Self::APP_FOLDER.clone();
        path.push("readme");
        path
    });
    
    pub const ENV_URL: &'static str = "https://pastebin.com/raw/wZGaNtsL";

}
