use reqwest;
use tokio::fs as TkFs;

use std::{
    fs::File,
    io::Write,
    path::Path,
    error::Error,
};

use crate::{
    consts::global::Global,
    system::system::System,

    ui::{
        errors_alerts::ErrorsAlerts,
        success_alerts::SuccessAlerts,
    },
};

pub struct DownloadConfigsFiles;

impl DownloadConfigsFiles {
    
    pub async fn env_file(print: bool, force_mode: bool) -> Result<(), Box<dyn Error>> {
        let url = Global::ENV_URL;
        let output_directory = &*System::APP_FOLDER;
    
        TkFs::create_dir_all(
            output_directory.clone()
        ).await?;
    
        let response = reqwest::get(url).await?;
        if response.status().is_success() {
            let file_path = output_directory.join(".env");
    
            if !force_mode {
                if !Path::new(&file_path).is_file() {
                    let mut file = File::create(&file_path)?;
                    let content = response.bytes().await?;
        
                    file.write_all(&content)?;
                }
            } else {
                let mut file = File::create(&file_path)?;
                let content = response.bytes().await?;
    
                file.write_all(&content)?;
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
      
    pub async fn settings_file(print: bool, force_mode: bool) -> Result<(), Box<dyn Error>> {
        let url = Global::SETTING_URL;
        let output_directory = &*System::APP_FOLDER;
    
        TkFs::create_dir_all(
            output_directory.clone()
        ).await?;
    
        let response = reqwest::get(url).await?;
        if response.status().is_success() {
            let file_path = output_directory.join("paimon.yml");
    
            if !force_mode {
                if !Path::new(&file_path).is_file() {
                    let mut file = File::create(&file_path)?;
                    let content = response.bytes().await?;
        
                    file.write_all(&content)?;
                }
            } else {
                let mut file = File::create(&file_path)?;
                let content = response.bytes().await?;
    
                file.write_all(&content)?;
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

}
