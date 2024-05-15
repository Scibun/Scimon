extern crate colored;

use regex::Regex;

use std::{
    fs,
    error::Error,
};

use pulldown_cmark::{
    html,
    Parser,
    Options,
};

use crate::{
    regex::regex_macros::MacrosRegExp,
    render::extra::render_extra_qrcode::RenderMarkdownExtraQrCode,
};

pub struct Markdown;

impl Markdown {

    async fn get_markdown_content(url: &str) -> Result<String, Box<dyn Error>> {
        let response = reqwest::get(url).await?;
        let content = response.text().await?;
        Ok(content)
    }

    pub async fn render_core(url: &str) -> Result<String, Box<dyn Error>> {
        let markdown_content = Self::get_markdown_content(url).await?;
    
        let options = Options::empty();
        let parser = Parser::new_ext(&markdown_content, options);

        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
    
        Ok(html_output)
    }

    pub fn render_readme(file: &str) -> Option<String> {
        let contents = fs::read_to_string(&file).expect("");
    
        let start_regex = Regex::new(MacrosRegExp::GET_README[0]).unwrap();
        let end_regex = Regex::new(MacrosRegExp::GET_README[1]).unwrap();

        if start_regex.is_match(&contents) && end_regex.is_match(&contents) {
            let start_match = start_regex.find(&contents).unwrap();
            let end_match = end_regex.find(&contents).unwrap();
        
            let start_index = start_match.start();
            let end_index = end_match.start() + "!end_readme".len();
        
            let markdown_block = &contents[start_index..end_index];
            let markdown_block_extra = &RenderMarkdownExtraQrCode::generate(markdown_block);
            let parser = Parser::new_ext(&markdown_block_extra, Options::all());
        
            let mut html_output = String::new();
            html::push_html(&mut html_output, parser);
        
            Some(html_output)
        } else {
            None
        }
    }

}
