extern crate open;

use regex::Regex;

use crate::regexp::regex_blocks::BlocksRegExp;

pub struct VarsBlock;

impl VarsBlock {

    pub fn get_path(contents: &str) -> String {
        let path_pattern = Regex::new(BlocksRegExp::GET_PATH_VAR).unwrap();

        let path = path_pattern.captures(&contents)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str())
            .unwrap_or_else(|| panic!("'path' variable not found in the file."));

        path.to_string()
    }

    pub async fn get_open(contents: &str, no_open: bool) -> Option<String> {
        if !no_open {
            let open_pattern = Regex::new(BlocksRegExp::GET_OPEN_VAR).unwrap();
        
            if let Some(caps) = open_pattern.captures(&contents) {
                let url = caps.get(1).map(|m| m.as_str().to_string())?;
                open::that(&url).unwrap();
                
                None
            } else {
                None
            }
        } else {
            None
        }
    }

    pub async fn get_readme(contents: &str) -> Option<String> {
        let readme_pattern = Regex::new(BlocksRegExp::GET_README_VAR).unwrap();
    
        if let Some(caps) = readme_pattern.captures(&contents) {
            caps.get(1).map(|m| m.as_str().to_string())
        } else {
            None
        }
    }

    pub fn get_compress(contents: &str) -> Option<String> {
        let readme_pattern = Regex::new(BlocksRegExp::GET_COMPRESS_VAR).unwrap();
    
        if let Some(caps) = readme_pattern.captures(&contents) {
            caps.get(1).map(|m| m.as_str().to_string())
        } else {
            None
        }
    }

    pub async fn get_checksum(contents: &str) -> Option<String> {
        let readme_pattern = Regex::new(BlocksRegExp::GET_CHECKSUM_VAR).unwrap();
    
        if let Some(caps) = readme_pattern.captures(&contents) {
            caps.get(1).map(|m| m.as_str().to_string())
        } else {
            None
        }
    }

    pub fn get_checksum_unmatch(contents: &str) -> Option<String> {
        let readme_pattern = Regex::new(BlocksRegExp::GET_CHECKSUM_UNMATCH_ACTION).unwrap();
    
        if let Some(caps) = readme_pattern.captures(&contents) {
            caps.get(1).map(|m| m.as_str().to_string())
        } else {
            None
        }
    }

}
