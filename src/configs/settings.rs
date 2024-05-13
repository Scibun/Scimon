use std::{
    fs::File,
    io::Read,
    error::Error,
};

use serde_yaml::{
    Value,
    from_str,
};

use crate::system::system::System;

pub struct Settings;

impl Settings {

    fn search_property_in_file(prop: &str) -> Result<String, Box<dyn Error>> {
        let mut file = File::open(&*System::SETTINGS_FILE)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        for (line_number, line) in contents.lines().enumerate() {
            if line.contains(prop.split('.').last().unwrap()) {
                return Ok(format!("{} -> '{}'", line_number + 1, line.trim()));
            }
        }

        Err(
            format!("Property '{}' not found in file.", prop).into()
        )
    }

    fn get_wrong_property_position(prop: &str, open_file: bool) -> Result<String, Box<dyn Error>> {
        let line_position = Self::search_property_in_file(prop)?;

        if open_file {
            let line_number = line_position.split(" -> ").next().unwrap();

            return Ok(
                format!(
                    "{}:{}", &System::SETTINGS_FILE.display(), line_number
                )
            );
        }

        Ok(
            format!(
                "Please fix it in: {}:{}", &System::SETTINGS_FILE.display(), line_position
            )
        )
    }

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
            let property_position = Self::get_wrong_property_position(prop, false)?;
            return Err(
                format!(
                    "The '{}' configuration is invalid. Expected type {}, but instead a {} was passed. {}.",
                    prop, data_type, value_type, property_position
                ).into()
            );
        }

        Ok(value.clone())
    }

    fn get_value(prop: &str, data_type: &str) -> Result<Value, Box<dyn Error>> {
        let mut file = File::open(&*System::SETTINGS_FILE)?;
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

}
