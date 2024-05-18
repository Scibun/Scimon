use std::fs;

use crate::{
    utils::file::FileMisc,
    system::system::System,
    configs::settings::Settings,
};

pub struct PrimeDownIO;

impl PrimeDownIO {

    fn get_path() -> String {
        let value = Settings::get("render_markdown.output_path", "STRING");

        let value_str = match value {
            serde_yaml::Value::String(s) => s.to_string(),
            _ => String::new()
        };

        value_str.replace(
            "{app_path}", System::README_FOLDER.to_str().unwrap_or_default()
        ).to_string()
    }

    pub fn get_file_path(file: &str) -> String {
        let path = Self::get_path();

        if !FileMisc::check_path_exists(&path) {
            let _ = fs::create_dir(&path);
        }

        format!(
            "{}\\{}", path, &file.replace(
                ".txt", ".html"
            )
        )
    }

}
