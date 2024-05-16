extern crate colored;

use reqwest;
use serde_yaml::Value::String as SerdeValue;

use std::{
    env,
    sync::Once,
    error::Error,
    process::Command,
    io::Error as IoError,

    path::{
        Path,
        PathBuf,
    }
};

use tokio::{
    io::AsyncWriteExt,

    fs::{
        File,
        self as TkFs,
    },
};

use crate::{
    consts::global::Global,
    system::system::System,
    configs::settings::Settings,

    ui::{
        errors_alerts::ErrorsAlerts,
        success_alerts::SuccessAlerts,
    },
};

pub struct Env;

impl Env {
    
    pub fn env_var(key: &str) -> String {
        let load_env: Once = Once::new();

        load_env.call_once(|| {
            dotenv::from_path(
                &System::APP_FOLDER.join(".env")
            ).ok();
        });
    
        env::var(key).expect(&format!("{} not set", key))
    }
    
    pub fn open_env_file() -> Result<(), IoError> {
        let app_folder = &*System::APP_FOLDER;
        let env_path: PathBuf = app_folder.join(".env");

        if let SerdeValue(editor) = &Settings::get(
            "general.default_text_editor", "STRING"
        ) {
            Command::new(
                editor
            ).arg(
                env_path
            ).spawn()?;
        }
        
        Ok(())
    }
    
    pub async fn download_env_file(print: bool, force_mode: bool) -> Result<(), Box<dyn Error>> {
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
                    let mut file = File::create(&file_path).await?;
                    let content = response.bytes().await?;
        
                    file.write_all(&content).await?;
                }
            } else {
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
   
}
