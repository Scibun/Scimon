extern crate colored;

use reqwest;
use std::env;
use tokio::fs::File;
use std::sync::Once;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use tokio::io::AsyncWriteExt;

use colored::*;

use crate::configs::global::{
    ENV_URL,
    APP_FOLDER
};

use crate::utils::misc::date_time;

static LOAD_ENV: Once = Once::new();

fn load_env_from_app_config() {
    let app_folder = &*APP_FOLDER;
    let env_path: PathBuf = app_folder.join(".env");
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

pub fn open_env_file() -> Result<(), std::io::Error> {
    let app_folder = &*APP_FOLDER;
    let env_path: PathBuf = app_folder.join(".env");

    Command::new("notepad.exe")
        .arg(env_path)
        .spawn()?;

    Ok(())
}

pub async fn force_download_env_file() -> Result<(), Box<dyn std::error::Error>> {
    let url = ENV_URL;
    let output_directory = &*APP_FOLDER;

    let cloned_output_directory = output_directory.clone();

    tokio::fs::create_dir_all(cloned_output_directory).await?;

    let response = reqwest::get(url).await?;
    if response.status().is_success() {
        let file_path = output_directory.join(".env");

        let mut file = File::create(&file_path).await?;
        let content = response.bytes().await?;

        file.write_all(&content).await?;
        println!("[{}] Downloaded env file", date_time().blue());
    } else {
        eprintln!("Failed to download the file: {:?}", response.status());
    }

    Ok(())
}

pub async fn download_env_file(print: bool) -> Result<(), Box<dyn std::error::Error>> {
    let url = ENV_URL;
    let output_directory = &*APP_FOLDER;

    let cloned_output_directory = output_directory.clone();

    tokio::fs::create_dir_all(cloned_output_directory).await?;

    let response = reqwest::get(url).await?;
    if response.status().is_success() {
        let file_path = output_directory.join(".env");

        if !check_file_exists(&file_path) {
            let mut file = File::create(&file_path).await?;
            let content = response.bytes().await?;

            file.write_all(&content).await?;
        }

        if print == true {
            println!("[{}] Downloaded env file", date_time().blue());
        }
    } else {
        eprintln!("Failed to download the file: {:?}", response.status());
    }

    Ok(())
}

pub async fn options_parser(options: &str) -> Result<(), Box<dyn std::error::Error>> {
    if options == "open-env" {
        let _ = open_env_file();
    } else if options == "force-download-env" {
        force_download_env_file().await?;
    }
    
    Ok(())
}
