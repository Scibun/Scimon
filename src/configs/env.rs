use serde_yaml::Value::String as SerdeValue;

use std::{
    env,
    sync::Once,
    path::PathBuf,
    process::Command,
    io::Error as IoError,
};

use crate::{
    system::system::System,
    configs::settings::Settings,
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
  
}
