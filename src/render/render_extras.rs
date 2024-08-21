use regex::Regex;

use crate::{
    generator::qr_code::GenQrCode,
    regexp::regex_core::CoreRegExp,
};

pub struct RenderExtras;

impl RenderExtras {
   
    pub fn gist(markdown: &str) -> String {
        let re = Regex::new(CoreRegExp::RENDER_EXTRA_GIST).unwrap();
    
        let replaced_markdown = re.replace_all(markdown, |captures: &regex::Captures| {
            let link = captures.get(1).unwrap().as_str();
            format!("<script src='{}.js'></script>", link)
        });
    
        replaced_markdown.to_string()
    }
   
    pub fn qrcode(markdown: &str) -> String {
        let re = Regex::new(CoreRegExp::RENDER_EXTRA_QRCODE).unwrap();
    
        let replaced_markdown = re.replace_all(markdown, |captures: &regex::Captures| {
            let link = captures.get(1).unwrap().as_str();
            let size: u32 = captures.get(2).unwrap().as_str().parse().unwrap();
    
            GenQrCode::new(link, size).html()
        });
    
        replaced_markdown.to_string()
    }
    
}
