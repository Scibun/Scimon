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

    pub async fn get_open(contents: &str) -> Option<String> {
        let open_pattern = Regex::new(BlocksRegExp::GET_OPEN_VAR).unwrap();
    
        if let Some(caps) = open_pattern.captures(&contents) {
            let link = caps.get(1).map(|m| m.as_str().to_string());
            
            if let Some(url) = link.clone() {
                let _ = open::that(&url);
            }
            
            link
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

    pub async fn get_checksum(contents: &str) -> Option<String> {
        let readme_pattern = Regex::new(BlocksRegExp::GET_CHECKSUM_VAR).unwrap();
    
        if let Some(caps) = readme_pattern.captures(&contents) {
            caps.get(1).map(|m| m.as_str().to_string())
        } else {
            None
        }
    }

}
