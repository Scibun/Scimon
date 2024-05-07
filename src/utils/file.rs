use uuid::Uuid;

use std::{
    fs,
    error::Error,

    path::{
        Path, 
        PathBuf
    }
};

use reqwest::{
    Url,
    self,
    header::HeaderValue
};

pub struct FileMisc;

impl FileMisc {

    pub fn check_path_exists(path: &str) -> bool {
        Path::new(&path).exists()
    }

    pub fn get_output_path(path: &str, filename: &str) -> PathBuf {
        let file_path = format!("{}/{}", path, filename);
        Self::clean_path(&file_path)
    }

    pub fn clean_path(path: &str) -> PathBuf {
        let trimmed_path = path.trim();
        let cleaned_path = trimmed_path.replace(" ", "");
    
        let mut path_buf = PathBuf::new();
        for component in cleaned_path.split('/') {
            if !component.is_empty() && component != "." && component != ".." {
                path_buf.push(component);
            }
        }
    
        path_buf
    }
    
    pub fn is_file_over(path: &str, size: u32) -> bool {
        let path = Path::new(path);
    
        if path.is_file() {
            if let Ok(metadata) = fs::metadata(path) {
                let file_size = metadata.len();
                file_size > (size * 1024 * 1024) as u64
            } else {
                false
            }
        } else {
            false
        }
    }
    
    pub fn get_file_name(path: &str) -> Result<String, &'static str> {
        Path::new(path)
            .file_name()
            .and_then(|name| name.to_str())
            .map(|s| s.to_string())
            .ok_or("Failed to get the file name.")
    }
    
    pub fn check_file_exists(path: &str) -> Result<(), &'static str> {
        let metadata = fs::metadata(
            Path::new(path)
        );
        
        if metadata.is_ok() && metadata.unwrap().is_file() {
            Ok(())
        } else {
            Err("The specified file does not exist.")
        }
    }

    pub async fn detect_name(url: &str, content_disposition: Option<&HeaderValue>) -> Result<String, Box<dyn Error>> {
        let filename_option = if let Some(value) = content_disposition {
            let cd_string = value.to_str()?;
            let parts: Vec<&str> = cd_string.split("filename=").collect();
    
            if parts.len() > 1 {
                Some(parts[1].trim_matches('"').to_string())
            } else {
                None
            }
        } else {
            let parsed_url = Url::parse(url)?;
            parsed_url.path_segments()
                .and_then(|segments| segments.last())
                .map(|name| name.to_string())
        };
    
        let final_filename = if let Some(ref filename) = filename_option {
            if !filename.contains(".pdf") {
                filename.clone() + ".pdf"
            } else {
                filename.clone()
            }
        } else {
            format!(
                "{}.pdf", Uuid::new_v4().to_string()
            )
        };
        
        Ok(final_filename)
    }

}
