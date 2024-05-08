extern crate colored;

use colored::*;
use regex::Regex;
use minify::html::minify;

use std::fs;

use pulldown_cmark::{
    html,
    Parser,
    Options,
};

use crate::render::{
    render_env::RenderMarkdownEnv,
    render_inject_js::RenderMarkdownInjectJS,
    render_inject_css::RenderMarkdownInjectCSS,
};

pub struct RenderMarkdownUtils;

impl RenderMarkdownUtils {

    fn remove_readme_macros(markdown_html: String) -> String {
        markdown_html.replace(
            "<p>!readme</p>\n", ""
        ).replace(
            "<p>!end_readme</p>\n", ""
        )
    }

    fn inject_content_template_file(file: &str, contents: String, markdown_html: String) -> String {
        let js_files_string = &RenderMarkdownInjectJS::load_from_files();
        let js_cdn_files_string = &RenderMarkdownInjectJS::load_from_cdn();

        let css_files_string = &RenderMarkdownInjectCSS::load_from_files();
        let css_cdn_files_string = &RenderMarkdownInjectCSS::load_from_cdn();

        let title = format!("{}: {}: README", &RenderMarkdownEnv::README_APP_NAME, &file);
        
        contents.replace(
            "{{ page_title }}", &title
        ).replace(
            "{{ inject_css_cdn }}", &css_cdn_files_string
        ).replace(
            "{{ inject_css_files }}", &css_files_string
        ).replace(
            "{{ markdown_content }}", &markdown_html
        ).replace(
            "{{ inject_js_cdn }}", &js_cdn_files_string
        ).replace(
            "{{ inject_js_files }}", &js_files_string
        )
    }

    pub fn render_markdown(file: &str) -> Option<String> {
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

    pub fn render_content(file: &str, markdown_html: String) -> String {
        let contents = fs::read_to_string(
            RenderMarkdownEnv::README_TEMPLATE_FILE
        ).expect(
            &"Unable to read readme.html file".to_string().red()
        );
        
        let markdown_html = Self::remove_readme_macros(markdown_html);
        let content = Self::inject_content_template_file(&file, contents, markdown_html);
        minify(&content)
    }
    
}
