use reqwest;
use std::env;
use tokio::fs::File;
use std::sync::Once;
use std::path::Path;
use std::path::PathBuf;
use tokio::io::AsyncWriteExt;

use crate::configs::global::{
    ENV_URL,
    APP_NAME
};

static LOAD_ENV: Once = Once::new();

fn load_env_from_app_config() {
    let config_dir = dirs_next::config_dir().expect("No config directory").join(APP_NAME);
    let env_path: PathBuf = config_dir.join(".env");
    dotenv::from_path(&env_path).ok();
}

fn check_file_exists(file_path: &PathBuf) -> bool {
    let path = Path::new(file_path);
    path.is_file()
}

pub fn env_var(key: &str) -> String {
    LOAD_ENV.call_once(|| {
        load_env_from_app_config();
    });

    env::var(key).expect(
        &format!("{} not set", key)
    )
}

pub async fn download_env_file() -> Result<(), Box<dyn std::error::Error>> {
    let url = ENV_URL;
    let output_directory = dirs_next::config_dir().expect("No config directory").join(APP_NAME);

    let cloned_output_directory = output_directory.clone();

    tokio::fs::create_dir_all(cloned_output_directory).await?;

    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let file_name = url
            .split('/')
            .last()
            .unwrap_or("downloaded_file");

        let file_path = output_directory.join(file_name);

        if !check_file_exists(&file_path) {
            let mut file = File::create(&file_path).await?;
            let content = response.bytes().await?;

            file.write_all(&content).await?;
        }
    } else {
        eprintln!("Failed to download the file: {:?}", response.status());
    }

    Ok(())
}
