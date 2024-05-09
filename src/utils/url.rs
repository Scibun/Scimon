extern crate url;

use url::Url;
use regex::Regex;
use is_url::is_url;
use std::error::Error;

use crate::{
    configs::regex::RegExp,
    ui::ui_alerts::PaimonUIAlerts,
};

pub struct UrlMisc;

impl UrlMisc {

    pub fn get_domain(url: &str) -> String {
        let url = Url::parse(url).expect("");
        url.host_str().expect("").to_owned()
    }

    // pub fn get_protocol(url: &str) -> String {
    //     let url = Url::parse(url).expect("Failed to parse URL");
    //     url.scheme().to_string()
    // }

    pub fn extract_url(line: &str) -> String {
        let re = Regex::new(RegExp::EXTRACT_URL).unwrap();

        if let Some(capture) = re.find(line) {
            capture.as_str().to_string()
        } else {
            String::new()
        }
    }

    pub fn escape_quotes(url: &str) -> String {
        url.replace("\"", "%22")
    }

    pub fn get_subdomain(url: &str) -> String {
        let url = Url::parse(url).expect("");
        let host = url.host_str().expect("");
        host.split('.').next().expect("").to_owned()
    }
 
    pub fn get_last_part(line: &str) -> String {
        let parts: Vec<&str> = line.split('/').collect();

        if let Some(last_part) = parts.last() {
            last_part.to_string()
        } else {
            String::new()
        }
    }

    pub fn open_url(url: &str, extract_url: bool) {
        let open_url = if extract_url {
            Self::extract_url(url)
        } else {
            url.to_string()
        };

        let _ = webbrowser::open(&open_url);
    }

    pub async fn get_status_code(url: &str) -> u16 {
        reqwest::get(url)
            .await
            .map(|response| response.status().as_u16())
            .unwrap_or(0)
    }

    pub fn check_domain(line: &str, domain: &str) -> bool {
        line.contains(domain)
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
        let regex = Regex::new(RegExp::VALIDATE_TAGS).unwrap();

        if regex.is_match(url) && !url.contains("*") && !url.is_empty() {
            let mut url_valid = false;
        
            if !is_url(url) {
                let url_invalid = Box::from("Invalid URL provided. Please enter a valid URL");
                PaimonUIAlerts::error_download(url_invalid, url);
            } else {
                url_valid = true;
            }

            if UrlMisc::get_status_code(url).await != 200 && url_valid == true {
                let status_code = Box::from("Failed to retrieve the URL with status code other than 200");
                PaimonUIAlerts::error_download(status_code, url);
            }
        }
        
        Ok(())
    }

}
