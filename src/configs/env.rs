extern crate colored;

use reqwest;

use std::{
    env,
    sync::Once,
    error::Error,
    process::Command,

    path::{
        Path,
        PathBuf,
    }
};

use tokio::{
    fs::File,
    io::AsyncWriteExt
};

use crate::{
    consts::global::Global,
    system::system::System,

    ui::{
        errors_alerts::ErrorsAlerts,
        success_alerts::SuccessAlerts,
    },
};

static LOAD_ENV: Once = Once::new();

pub struct Env;

impl Env {

    fn load_env_from_app_config() {
        let app_folder = &*System::APP_FOLDER;
        let env_path = app_folder.join(".env");
        dotenv::from_path(&env_path).ok();
    }
    
    fn check_file_exists(file_path: &PathBuf) -> bool {
        let path = Path::new(file_path);
        path.is_file()
    }
    
    pub fn env_var(key: &str) -> String {
        LOAD_ENV.call_once(|| {
            Self::load_env_from_app_config();
        });
    
        env::var(key).expect(
            &format!("{} not set", key)
        )
    }
    
    pub fn open_env_file() -> Result<(), std::io::Error> {
        let app_folder = &*System::APP_FOLDER;
        let env_path: PathBuf = app_folder.join(".env");
    
        Command::new("notepad.exe")
            .arg(env_path)
            .spawn()?;
    
        Ok(())
    }
    
    pub async fn force_download_env_file() -> Result<(), Box<dyn Error>> {
        let url = Global::ENV_URL;
        let output_directory = &*System::APP_FOLDER;
        let cloned_output_directory = output_directory.clone();
    
        tokio::fs::create_dir_all(cloned_output_directory).await?;
    
        let response = reqwest::get(url).await?;
        if response.status().is_success() {
            let file_path = output_directory.join(".env");
    
            let mut file = File::create(&file_path).await?;
            let content = response.bytes().await?;
    
            file.write_all(&content).await?;
            SuccessAlerts::env();
        } else {
            let status_code = response.status().to_string();
            ErrorsAlerts::env(&status_code);
        }
    
        Ok(())
    }
    
    pub async fn download_env_file(print: bool) -> Result<(), Box<dyn Error>> {
        let url = Global::ENV_URL;
        let output_directory = &*System::APP_FOLDER;
    
        let cloned_output_directory = output_directory.clone();
    
        tokio::fs::create_dir_all(cloned_output_directory).await?;
    
        let response = reqwest::get(url).await?;
        if response.status().is_success() {
            let file_path = output_directory.join(".env");
    
            if !Self::check_file_exists(&file_path) {
                let mut file = File::create(&file_path).await?;
                let content = response.bytes().await?;
    
                file.write_all(&content).await?;
            }
    
            if print == true {
                SuccessAlerts::env();
            }
        } else {
            let status_code = response.status().to_string();
            ErrorsAlerts::env(&status_code);
        }
    
        Ok(())
    }
    
    pub async fn options_parser(options: &str) -> Result<(), Box<dyn Error>> {
        if options == "open-env" {
            let _ = Self::open_env_file();
        } else if options == "force-download-env" {
            Self::force_download_env_file().await?;
        }
        
        Ok(())
    }
    
}
