extern crate colored;

use colored::*;
use regex::Regex;

use std::{
    fs,
    env, 
};

use pulldown_cmark::{
    html,
    Parser,
    Options,
};

use crate::{
    configs::global::Global,
    render::render_env::RenderEnv,

    utils::{
        url::UrlMisc,
        file::FileMisc,
    }
};

pub struct RenderMarkdown;

impl RenderMarkdown {

    fn get_output_path() -> String {
        let path_buf = &*RenderEnv::README_FOLDER;
        let path = path_buf.to_str().unwrap();

        if !FileMisc::check_path_exists(&path) {
            let _ = fs::create_dir(&path);
        }

        path.to_owned()
    }

    fn render_markdown(file: &str) -> Option<String> {
        let contents = fs::read_to_string(&file).expect("Unable to read file");
    
        let start_regex = Regex::new(r"!readme").unwrap();
        let end_regex = Regex::new(r"!end_readme").unwrap();

        if start_regex.is_match(&contents) && end_regex.is_match(&contents) {
            let start_match = start_regex.find(&contents).unwrap();
            let end_match = end_regex.find(&contents).unwrap();
        
            let start_index = start_match.start();
            let end_index = end_match.start() + "!end_readme".len();
        
            let markdown_block = &contents[start_index..end_index];
            let parser = Parser::new_ext(&markdown_block, Options::all());
        
            let mut html_output = String::new();
            html::push_html(&mut html_output, parser);
        
            Some(html_output)
        } else {
            None
        }
    }

    fn open_readme_file_url(output_path: &str, no_open_link: bool) {
        if !no_open_link {
            let full_path = env::current_dir().expect(
                &"Error getting current working directory".to_string().red()
            ).join(&output_path).to_str().unwrap().replace(
                "\\", "/"
            );

            let url_file = &format!("file://{}", full_path);
            UrlMisc::open_url(&url_file, false);
        }
    }

    fn render_content(markdown_html: String, file: &str) -> String {
        let contents = fs::read_to_string(
            RenderEnv::README_TEMPLATE_FILE
        ).expect(
            &"Unable to read readme.html file".to_string().red()
        );
        
        let markdown_html = markdown_html.replace(
            "<p>!readme</p>\n", ""
        ).replace(
            "<p>!end_readme</p>\n", ""
        );

        contents.replace(
            "{{ css_file_base }}", &RenderEnv::README_CSS_BASE_FILE
        ).replace(
            "{{ app_name }}", &Global::APP_NAME
        ).replace(
            "{{ list_name }}", &file
        ).replace(
            "{{ markdown_content }}", &markdown_html
        )
    }

    pub fn render_and_save_file(file: &str, no_open_link: bool) {
        if let Some(markdown_html) = Self::render_markdown(file) {
            let contents = Self::render_content(
                markdown_html, &file
            );

            let output_path = format!(
                "{}\\{}", Self::get_output_path(), &file.replace(
                    ".txt", ".html"
                )
            );
    
            fs::write(&output_path, contents).expect(
                &"Error saving HTML file".to_string().red()
            );

            Self::open_readme_file_url(&output_path, no_open_link)
        }
    }
    
}
