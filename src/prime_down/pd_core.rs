extern crate colored;

use std::fs;
use colored::*;
use regex::Regex;
use minify::html::minify;

use crate::{
    configs::settings::Settings,
    consts::render::RenderMarkdownEnv,
    regex::regex_macros::MacrosRegExp,

    system::{
        syntax::Macros,
        markdown::Markdown,
    },

    prime_down::{
        pd_io::PrimeDownIO,
        injection::pd_inject::PrimeDownInject,
    }
};

pub struct PrimeDown;

impl PrimeDown {

    fn render_content(file: &str, md_content: String) -> String {
        let minify_prop = Settings::get("render_markdown.minify_html", "BOOLEAN");

        let contents = fs::read_to_string(
            RenderMarkdownEnv::README_TEMPLATE_FILE
        ).expect(
            &"Unable to read readme.html file".to_string().red()
        );
        
        let markdown_html = Macros::remove_readme_macros_html(md_content);
        let content = PrimeDownInject::content(&file, contents, markdown_html);

        if minify_prop == true {
            minify(&content)
        } else {
            content
        }
    }

    pub fn render_and_save_file(file: &str, no_open_link: bool) {
        if let Some(markdown_html) = Markdown::render_readme(file) {
            let path = PrimeDownIO::get_file_path(file);
            let contents = Self::render_content(&file, markdown_html);

            PrimeDownIO::write_file(&path, contents);
            PrimeDownIO::open_readme_url(&path, no_open_link)
        }
    }
    
    pub fn start_end_macros_position() -> Result<(Regex, Regex), String> {
        let start_regex = Regex::new(MacrosRegExp::GET_README[0])
            .map_err(|e| format!("Failed to compile start regex: {}", e))?;
        let end_regex = Regex::new(MacrosRegExp::GET_README[1])
            .map_err(|e| format!("Failed to compile end regex: {}", e))?;
    
        Ok((start_regex, end_regex))
    }

}
