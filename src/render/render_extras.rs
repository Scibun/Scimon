use regex::Regex;

use crate::{
    utils::generate::Generate,
    configs::settings::Settings,
    regex::regex_core::CoreRegExp,
};

pub struct RenderMarkdownExtras;

impl RenderMarkdownExtras {
   
    pub fn gist(markdown: &str) -> String {
        if Settings::get("render_markdown.mode", "STRING") == "paimon" {
            let re = Regex::new(CoreRegExp::RENDER_EXTRA_GIST).unwrap();
        
            let replaced_markdown = re.replace_all(markdown, |captures: &regex::Captures| {
                let link = captures.get(1).unwrap().as_str();
                format!("<script src='{}.js'></script>", link)
            });
        
            replaced_markdown.to_string()
        } else {
            markdown.to_string()
        }
    }
   
    pub fn qrcode(markdown: &str) -> String {
        if Settings::get("render_markdown.mode", "STRING") == "paimon" {
            let re = Regex::new(CoreRegExp::RENDER_EXTRA_QRCODE).unwrap();
        
            let replaced_markdown = re.replace_all(markdown, |captures: &regex::Captures| {
                let link = captures.get(1).unwrap().as_str();
                let size: u32 = captures.get(2).unwrap().as_str().parse().unwrap();
        
                let qr_code_base64 = Generate::qrcode(link, size);
                format!("![QR Code](data:image/png;base64,{})", qr_code_base64)
            });
        
            replaced_markdown.to_string()
        } else {
            markdown.to_string()
        }
    }
    
}
