extern crate colored;

use colored::*;
use regex::Regex;
use minify::html::minify;

use std::{
    fs,
    error::Error,
};

use headless_chrome::{
    Browser, 
    LaunchOptionsBuilder,
    types::PrintToPdfOptions,
};

use crate::{
    utils::base64::Base64,
    system::syntax::Macros,
    configs::settings::Settings,
    consts::prime_down::PrimeDownEnv,
    regexp::regex_macros::MacrosRegExp,
    prime_down::inject::pd_inject::PrimeDownInject,
};

pub struct PrimeDownMisc;

impl PrimeDownMisc {

    pub fn render_content(file: &str, md_content: String) -> String {
        let minify_prop = Settings::get("render_markdown.minify_html", "BOOLEAN");

        let contents = fs::read_to_string(
            PrimeDownEnv::README_TEMPLATE_FILE
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
   
    pub fn start_end_macros_position() -> Result<(Regex, Regex), String> {
        let start_regex = Regex::new(MacrosRegExp::GET_README[0])
            .map_err(|e| format!("Failed to compile start regex: {}", e))?;
        let end_regex = Regex::new(MacrosRegExp::GET_README[1])
            .map_err(|e| format!("Failed to compile end regex: {}", e))?;
    
        Ok((start_regex, end_regex))
    }

    pub async fn connect_to_browser(content: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let browser = Browser::new(
            LaunchOptionsBuilder::default().build().expect(""),
        )?;

        let pdf_options: Option<PrintToPdfOptions> = None;

        let contents = browser.new_tab()?.navigate_to(
            &Base64::encode_html(content)
        )?.wait_until_navigated()?.print_to_pdf(
            pdf_options
        )?;

        Ok(contents)
    }

}
