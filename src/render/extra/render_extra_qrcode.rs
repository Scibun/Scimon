use image::Luma;
use regex::Regex;
use qrcode::QrCode;
use std::io::Cursor;
use base64::prelude::*;

use crate::{
    configs::settings::Settings,
    regex::core_regex::CoreRegExp,
};

pub struct RenderMarkdownExtraQrCode;

impl RenderMarkdownExtraQrCode {

    fn generate_qr_code_to_base64(link: &str, size: u32) -> String {
        let code = QrCode::new(link).unwrap();
        let image = code.render::<Luma<u8>>().max_dimensions(size, size).build();
    
        let mut img_bytes = Vec::new();
        let mut cursor = Cursor::new(&mut img_bytes);
        image.write_to(&mut cursor, image::ImageFormat::Png).unwrap();
    
        let img_base64 = BASE64_STANDARD.encode(&img_bytes);
        img_base64
    }
    
    pub fn generate(markdown: &str) -> String {
        if Settings::get("render_markdown.mode", "STRING") == "paimon" {
            let re = Regex::new(CoreRegExp::RENDER_EXTRA_QRCODE).unwrap();
        
            let replaced_markdown = re.replace_all(markdown, |captures: &regex::Captures| {
                let link = captures.get(1).unwrap().as_str();
                let size: u32 = captures.get(2).unwrap().as_str().parse().unwrap();
        
                let qr_code_base64 = Self::generate_qr_code_to_base64(link, size);
                format!("![QR Code](data:image/png;base64,{})", qr_code_base64)
            });
        
            replaced_markdown.to_string()
        } else {
            markdown.to_string()
        }
    }
    
}
