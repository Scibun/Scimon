use is_url::is_url;

use reqwest;
use std::fs;
use std::error::Error;

pub fn get_file_size(file: &str) -> Result<u64, Box<dyn Error>> {
    let metadata = fs::metadata(file)?;
    Ok(metadata.len())
}

pub fn format_bytes(bytes: u64) -> String {
    let units = [" bytes", "KB", "MB", "GB", "TB", "PB", "EB"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;

    while size >= 1024.0 && unit_idx < units.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }

    format!("{:.2} {}", size, units[unit_idx])
}

pub async fn get_remote_file_size(url: &str) -> Result<u64, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let response = client.head(url).send().await?;

    if response.status().is_success() {
        if let Some(content_length) = response.headers().get("Content-Length") {
            if let Ok(content_length_str) = content_length.to_str() {
                if let Ok(content_length) = content_length_str.parse::<u64>() {
                    return Ok(content_length);
                }
            }
        }

        return Err("'Content-Length' header was not found or is not a valid number.".into());
    } else {
        return Err(
            format!(
                "Error accessing the remote file: {:?}", response.status()
            ).into()
        );
    }
}

pub async fn get_file_size_format(file: &str) -> Result<String, Box<dyn Error>> {
    if is_url(file) {
        let size = get_remote_file_size(file).await?;
        Ok(format_bytes(size))
    } else {
        let size = get_file_size(file)?;
        Ok(format_bytes(size))
    }
}
