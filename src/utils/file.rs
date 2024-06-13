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

pub struct FileUtils;

impl FileUtils {

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

    pub fn create_path(path: &str) {
        if !Self::check_path_exists(path) {
            fs::create_dir_all(path).expect(
                &"Error creating directory".to_string()
            );
        }
    }
    
    pub fn write_file(path: &str, contents: String) {
        fs::write(path, contents).expect(
            &"Error saving file".to_string()
        );
    }

    pub fn set_final_filename(file: Option<String>) -> String {
        if let Some(ref filename) = file {
            if !filename.contains(".pdf") {
                filename.clone() + ".pdf"
            } else {
                filename.clone()
            }
        } else {
            format!(
                "{}.pdf", Uuid::new_v4().to_string()
            )
        }
    }

    pub fn get_output_path(path: &str, filename: &str) -> PathBuf {
        let file_path = format!("{}/{}", path, filename);
        Self::clean_path(&file_path)
    }

    pub async fn detect_name(url: &str, disposition: Option<&HeaderValue>, pdf: bool) -> Result<String, Box<dyn Error>> {
        let filename_option = if let Some(value) = disposition {
            let cd_string = value.to_str()?;
            let parts: Vec<&str> = cd_string.split("filename=").collect();
    
            if parts.len() > 1 {
                Some(
                    parts[1].trim_matches('"').to_string()
                )
            } else {
                None
            }
        } else {
            let parsed_url = Url::parse(url)?;
            parsed_url.path_segments()
                .and_then(|segments| segments.last())
                .map(|name| name.to_string())
        };
    
        let final_filename = if pdf == true {
            Self::set_final_filename(filename_option)
        } else {
            filename_option.unwrap()
        };
        
        Ok(final_filename)
    }

    pub fn replace_extension(file_name: &str, new_extension: &str) -> String {
        let mut path = PathBuf::from(file_name);

        if let Some(name) = path.file_name() {
            if let Some(name_str) = name.to_str() {
                if let Some(_) = name_str.rfind('.') {
                    path.set_extension(new_extension);
                    return path.to_string_lossy().to_string();
                }
            }
        }
        
        path.set_extension(new_extension);
        path.to_string_lossy().to_string()
    }

}
