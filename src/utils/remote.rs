extern crate url;

use reqwest;

use std::{
    fs::File,
    io::Write,
    error::Error,
};

use crate::utils::file::FileUtils;

pub struct Remote;

impl Remote {

    pub async fn get_status_code(url: &str) -> u16 {
        reqwest::get(url)
            .await
            .map(|response| response.status().as_u16())
            .unwrap_or(0)
    }

    pub async fn content(url: &str) -> Result<String, Box<dyn Error>> {
        let response = reqwest::get(url).await?;
        Ok(response.text().await?)
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

    pub async fn get_filename(url: &str, pdf: bool) -> Result<String, Box<dyn Error>> {
        let filename = FileUtils::detect_name(
            url, reqwest::get(url).await?.headers().get("content-disposition"), pdf
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

    pub async fn download(url: &str, path: &str) -> Result<String, Box<dyn Error>> {
        let response = reqwest::get(url).await?;
        
        if response.status().is_success() {
            let filename = Remote::get_filename(url, false).await?;
            
            let file_path = format!(
                "{}/{}", path, url.split("/").last().unwrap_or(&filename)
            );

            let mut file = File::create(&file_path)?;
            let content = response.bytes().await?;
    
            file.write_all(&content)?;
            return Ok(file_path)
        }

        Ok("".to_string())
    }

}
