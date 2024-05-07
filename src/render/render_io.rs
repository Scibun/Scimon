extern crate colored;

use colored::*;

use std::{
    fs,
    env
};

use crate::{
    render::render_env::RenderMarkdownEnv,
    
    utils::{
        url::UrlMisc,
        file::FileMisc
    },
};

pub struct RenderMarkdownIO;

impl RenderMarkdownIO {

    pub fn get_file_path(file: &str) -> String {
        let path_buf = &*RenderMarkdownEnv::README_FOLDER;
        let path = path_buf.to_str().unwrap();

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
                &"Error getting current working directory".to_string().red()
            ).join(&path).to_str().unwrap().replace(
                "\\", "/"
            );

            let url_file = &format!("file://{}", full_path);
            UrlMisc::open_url(&url_file, false);
        }
    }

    pub fn write_file(path: &str, contents: String) {
        fs::write(path, contents).expect(
            &"Error saving HTML file".to_string().red()
        );
    }

}
