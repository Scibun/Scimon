use regex::Regex;

use crate::regexp::regex_blocks::BlocksRegExp;

pub struct VarsBlock;

impl VarsBlock {

    pub fn get_path(contents: &str) -> String {
        let path_pattern = Regex::new(BlocksRegExp::GET_PATH).unwrap();
        let path = path_pattern.captures(&contents)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str())
            .unwrap_or_else(|| panic!("'path' variable not found in the file."));

        path.to_string()
    }
}
