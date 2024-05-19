use std::fs;
use serde_yaml::Value;

use crate::{
    consts::prime_down::PrimeDownEnv,
    prime_down::pd_minify::PrimeDownMinify,
};

pub struct PrimeDownInjectCSS;

impl PrimeDownInjectCSS {

    pub fn load_from_files(minify: Value) -> String {
        let mut content_css = String::new();
        let js_path = PrimeDownEnv::README_TEMPLATE_CSS_FILES;
    
        for entry in fs::read_dir(js_path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
    
            if path.is_file() && path.extension().map_or(false, |ext| ext == "css") {
                let css_content = fs::read_to_string(path).unwrap();
                let format_css_content = &format!("{}\n", &css_content);

                content_css.push_str(&format_css_content);
            }
        }
    
        content_css = if minify == true {
            PrimeDownMinify::css(&content_css)
        } else {
            content_css
        };

        format!(
            "<style>{}</style>", &content_css
        )
    }
    
}
