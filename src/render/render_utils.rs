extern crate colored;

use colored::*;
use minify::html::minify;

use std::fs;

use crate::{
    system::syntax::Macros,
    configs::settings::Settings,
    consts::render::RenderMarkdownEnv,
    render::injection::render_inject::RenderMarkdownInject,
};

pub struct RenderMarkdownUtils;

impl RenderMarkdownUtils {

    pub fn render_content(file: &str, markdown_html: String) -> String {
        let minify_prop = Settings::get("render_markdown.minify_html", "BOOLEAN");

        let contents = fs::read_to_string(
            RenderMarkdownEnv::README_TEMPLATE_FILE
        ).expect(
            &"Unable to read readme.html file".to_string().red()
        );
        
        let markdown_html = Macros::remove_readme_macros_html(markdown_html);
        let content = RenderMarkdownInject::content(&file, contents, markdown_html);

        if minify_prop == true {
            minify(&content)
        } else {
            content
        }
    }
    
}
