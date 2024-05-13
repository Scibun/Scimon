extern crate colored;

use colored::*;

use std::{
    fs,
    env
};

use crate::{
    system::system::System,
    configs::settings::Settings,

    utils::{
        url::UrlMisc,
        file::FileMisc,
    }
};

pub struct RenderMarkdownIO;

impl RenderMarkdownIO {

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

    pub fn open_readme_url(path: &str, no_open_link: bool) {
        if !no_open_link {
            let full_path = env::current_dir().expect(
                ""
            ).join(&path).to_str().unwrap().replace(
                "\\", "/"
            );

            let url_file = &format!(
                "file://{}", full_path
            );

            UrlMisc::open_url(&url_file, false);
        }
    }

    pub fn write_file(path: &str, contents: String) {
        fs::write(path, contents).expect(
            &"Error saving HTML file".to_string().red()
        );
    }

}
