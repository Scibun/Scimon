use std::path::PathBuf;
use once_cell::sync::Lazy;
use dirs_next::config_dir;

pub static APP_NAME: &str = "Paimon";
pub static APP_VERSION: &str = "0.0.1";
pub static APP_AUTHOR: &str = "@Kremilly";
pub static APP_HOMEPAGE: &str = "https://github.com/kremilly/Paimon";

pub static MONLIB_API_REQUEST: &str = "http://localhost/Monlib/api/";

// pub static API_USER_ENDPOINT: &str = "user";
pub static API_LISTS_ENDPOINT: &str = "lists";

pub static APP_FOLDER: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = config_dir().expect("No config directory");
    path.push(APP_NAME);
    path
});

pub static ENV_URL: &str = "https://pastebin.com/raw/wZGaNtsL";
