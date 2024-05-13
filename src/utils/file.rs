use uuid::Uuid;

use std::{
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

    pub fn check_path_exists(path: &str) -> bool {
        Path::new(&path).exists()
    }

    pub fn get_output_path(path: &str, filename: &str) -> PathBuf {
        let file_path = format!("{}/{}", path, filename);
        Self::clean_path(&file_path)
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
