use std::{
    fs,
    path::{Path, PathBuf}
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

}
