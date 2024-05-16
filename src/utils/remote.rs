extern crate url;

use reqwest;
use std::error::Error;

use crate::utils::file::FileMisc;

pub struct FileRemote;

impl FileRemote {

    pub async fn get_status_code(url: &str) -> u16 {
        reqwest::get(url)
            .await
            .map(|response| response.status().as_u16())
            .unwrap_or(0)
    }

    pub async fn content(url: &str) -> Result<String, Box<dyn Error>> {
        let response = reqwest::get(url).await?;
        let content = response.text().await?;
        Ok(content)
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

    pub async fn get_file_size(url: &str) -> Result<u64, Box<dyn Error>> {
        let response = reqwest::get(url).await?;
    
        let total_size = response
            .headers()
            .get(reqwest::header::CONTENT_LENGTH)
            .and_then(|ct_len| ct_len.to_str().ok())
            .and_then(|ct_len| ct_len.parse::<u64>().ok())
            .unwrap_or(0);

        Ok(total_size)
    }

    pub async fn get_filename(url: &str) -> Result<String, Box<dyn Error>> {
        let filename = FileMisc::detect_name(
            url, reqwest::get(url).await?.headers().get("content-disposition")
        ).await?;

        Ok(filename)
    }
    
    pub async fn check_content_type(url: &str, mime_type: &str) -> Result<bool, Box<dyn Error>> {
        let client: reqwest::Client = reqwest::Client::new();
        let response = client.get(url).send().await?;

        if !response.status().is_success() {
            return Ok(false);
        }

        if let Some(content_type) = response.headers().get(reqwest::header::CONTENT_TYPE) {
            if let Ok(content_type_str) = content_type.to_str() {
                if content_type_str == mime_type {
                    return Ok(true);
                }
            }
        }
    
        Ok(false)
    }

}
