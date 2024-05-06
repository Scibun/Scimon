use regex::Regex;
use std::error::Error;

use crate::configs::regex::RegExp;

pub struct UrlMisc;

impl UrlMisc {

    pub fn open_url(line: &str) {
        let get_url = Self::extract_url(line);
        let _ = webbrowser::open(&get_url);
    }

    pub fn extract_url(line: &str) -> String {
        let re = Regex::new(RegExp::EXTRACT_URL).unwrap();

        if let Some(capture) = re.find(line) {
            capture.as_str().to_string()
        } else {
            String::new()
        }
    }
    
    pub fn extract_file_name(url: &str) -> Option<String> {
        let url_cleaned = url.split(' ').next().unwrap_or_default();
        let re = Regex::new(RegExp::EXTRACT_PDF_NAME).expect("Invalid regex pattern");

        if let Some(captures) = re.captures(url_cleaned) {
            let parser_filename = captures.get(1).unwrap().as_str().to_string();

            let final_name = if !parser_filename.contains(".pdf") {
                parser_filename.clone() + ".pdf"
            } else {
                parser_filename.clone()
            };

            Some(final_name)
        } else {
            None
        }
    }
    
    pub async fn is_pdf_file(url: &str) -> Result<bool, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let response = client.get(url).send().await?;

        if !response.status().is_success() {
            return Ok(false);
        }

        if let Some(content_type) = response.headers().get(reqwest::header::CONTENT_TYPE) {
            if let Ok(content_type_str) = content_type.to_str() {
                if content_type_str == "application/pdf" {
                    return Ok(true);
                }
            }
        }
    
        Ok(false)
    }

}
