use serde_yaml::Value::String as SerdeValue;

use std::{
    fs::File,
    error::Error,
    path::PathBuf,

    io::{
        Read,
        Error as IoError,
    },
};

use serde_yaml::{
    Value,
    from_str,
};

use crate::configs::folders::Folders;

pub struct Settings;

impl Settings {

    fn is_valid(prop: &str, value: &Value, data_type: &str) -> Result<Value, Box<dyn Error>> {
        let value_type = match value {
            Value::String(_) => "STRING",
            Value::Number(_) => "INT",
            Value::Bool(_) => "BOOLEAN",
            Value::Sequence(_) => "LIST",
            _ => {
                return Err(
                    format!(
                        "Invalid type for property '{}'", prop
                    ).into()
                );
            }
        };

        if value_type != data_type {
            return Err(
                format!(
                    "The '{}' configuration is invalid. Expected type {}, but instead a {} was passed.",
                    prop, data_type, value_type
                ).into()
            );
        }

        Ok(value.clone())
    }

    fn get_value(prop: &str, data_type: &str) -> Result<Value, Box<dyn Error>> {
        let mut file = File::open(&*Folders::SETTINGS_FILE)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let data: Value = from_str(&contents)?;

        let mut value = &data;
        let property_parts: Vec<&str> = prop.split('.').collect();

        for part in &property_parts {
            value = &value[part];
        }

        Self::is_valid(prop, value, data_type)
    }

    pub fn get(prop: &str, data_type: &str) -> Value {
        match Self::get_value(prop, data_type) {
            Ok(value) => value,

            Err(e) => {
                eprintln!("{}", e);
                Value::Null
            }
        }
    }

    pub fn open_settings_file() -> Result<(), IoError> {
        let app_folder = &*Folders::APP_FOLDER;
        let env_path: PathBuf = app_folder.join("scibun.yml");

        if let SerdeValue(editor) = &Settings::get(
            "general.default_text_editor", "STRING"
        ) {
            open::with(env_path, editor)?;
        }
        
        Ok(())
    }

}
