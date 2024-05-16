extern crate colored;

use std::fs;
use colored::*;
use minify::html::minify;

use crate::{
    configs::settings::Settings,
    consts::render::RenderMarkdownEnv,

    system::{
        syntax::Macros,
        markdown::Markdown,
    },

    render::{
        render_io::RenderMarkdownIO,
        injection::render_inject::RenderMarkdownInject,
    }
};

pub struct RenderMarkdown;

impl RenderMarkdown {

    fn render_content(file: &str, md_content: String) -> String {
        let minify_prop = Settings::get("render_markdown.minify_html", "BOOLEAN");

        let contents = fs::read_to_string(
            RenderMarkdownEnv::README_TEMPLATE_FILE
        ).expect(
            &"Unable to read readme.html file".to_string().red()
        );
        
        let markdown_html = Macros::remove_readme_macros_html(md_content);
        let content = RenderMarkdownInject::content(&file, contents, markdown_html);

        if minify_prop == true {
            minify(&content)
        } else {
            content
        }
    }

    pub fn render_and_save_file(file: &str, no_open_link: bool) {
        if let Some(markdown_html) = Markdown::render_readme(file) {
            let path = RenderMarkdownIO::get_file_path(file);
            let contents = Self::render_content(&file, markdown_html);

            RenderMarkdownIO::write_file(&path, contents);
            RenderMarkdownIO::open_readme_url(&path, no_open_link)
        }
    }
    
}
