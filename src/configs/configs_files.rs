use reqwest;
use tokio::fs as TkFs;

use std::{
    fs::File,
    io::Write,
    path::Path,
    error::Error,
};

use crate::{
    consts::{
        global::Global,
        folders::Folders,
    },

    ui::{
        errors_alerts::ErrorsAlerts,
        success_alerts::SuccessAlerts,
    },
};

pub struct DownloadConfigsFiles;

impl DownloadConfigsFiles {
    
    pub async fn env_file(print: bool, force_mode: bool) -> Result<(), Box<dyn Error>> {
        let output_directory = &*Folders::APP_FOLDER;
        let uri = format!("{}{}", Global::DOWNLOAD_FILES_URI, ".env.example");
    
        TkFs::create_dir_all(
            output_directory.clone()
        ).await?;
    
        let response = reqwest::get(uri).await?;
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
        let output_directory = &*Folders::APP_FOLDER;
        let uri = format!("{}{}", Global::DOWNLOAD_FILES_URI, "scimon.yml");
    
        TkFs::create_dir_all(
            output_directory.clone()
        ).await?;
    
        let response = reqwest::get(uri).await?;
        if response.status().is_success() {
            let file_path = output_directory.join(
                format!("{}.yml", Global::APP_NAME.to_lowercase())
            );
    
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
