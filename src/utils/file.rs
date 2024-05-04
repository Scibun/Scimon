use std::{
    fs,
    path::Path,
};

pub struct FileUtils;

impl FileUtils {
    
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
    
    pub fn get_file_name_string(file_path: &str) -> String {
        let path = Path::new(file_path);
        
        path.file_name()
            .and_then(|name| name.to_str())
            .map(|s| s.to_owned())
            .unwrap_or_else(|| "unknown_filename".to_owned())
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
