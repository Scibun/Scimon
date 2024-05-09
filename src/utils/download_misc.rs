extern crate url;

use regex::Regex;
use is_url::is_url;
use std::error::Error;

use crate::{
    utils::url::UrlMisc,
    configs::regex::RegExp,
    ui::ui_alerts::PaimonUIAlerts,
};

pub struct DownloadMisc;

impl DownloadMisc {
    
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
        let client: reqwest::Client = reqwest::Client::new();
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

    pub async fn check_errors(url: &str) -> Result<(), Box<dyn Error>> {
        let final_url = &url.replace(" !ignore", "");
        let regex = Regex::new(RegExp::VALIDATE_TAGS).unwrap();

        if regex.is_match(final_url) && !final_url.contains("*") && !final_url.is_empty() {
            let mut url_valid = false;
        
            if !is_url(final_url) {
                let url_invalid = Box::from("Invalid URL provided. Please enter a valid URL");
                PaimonUIAlerts::error_download(url_invalid, final_url);
            } else {
                url_valid = true;
            }

            if UrlMisc::get_status_code(final_url).await != 200 && url_valid == true {
                let status_code = Box::from("Failed to retrieve the URL with status code other than 200");
                PaimonUIAlerts::error_download(status_code, final_url);
            }
        }
        
        Ok(())
    }

}
