use std::path::PathBuf;
use once_cell::sync::Lazy;
use dirs_next::config_dir;

pub struct Global;

impl Global {

    pub const APP_NAME: &'static str = "Paimon";
    pub const APP_VERSION: &'static str = "0.0.1";
    pub const APP_AUTHOR: &'static str = "@Ravenlib";
    pub const APP_HOMEPAGE: &'static str = "https://github.com/Ravenlib/Paimon";

    pub const PB_STYLE: &'static str = "[{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})";
    
    pub const APP_FOLDER: Lazy<PathBuf> = Lazy::new(|| {
        let mut path = config_dir().expect("No config directory");
        path.push(Self::APP_NAME);
        path
    });
    
    pub const SETTINGS_FILE: Lazy<PathBuf> = Lazy::new(|| {
        let mut path = config_dir().expect("No config directory");
        path.push(Self::APP_NAME);
        path.push("paimon.yml");
        path
    });     
    
    pub const ENV_URL: &'static str = "https://pastebin.com/raw/wZGaNtsL";

    pub const PROVIDERS_DOMAINS: [&'static str; 4] = ["wikipedia.org", "sci-hub.se", "github.com", "githubusercontent.com"];

}
