use rand::Rng;
use image::Luma;
use qrcode::QrCode;
use std::io::Cursor;

use crate::utils::base64::Base64;

pub struct Generate;

impl Generate {
    
    pub fn random_string(length: usize) -> String {
        let charset: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        let charset_len: usize = charset.len();
        let mut rng = rand::thread_rng();
    
        let random_string: String = (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..charset_len);
                charset.chars().nth(idx).unwrap()
            })
            .collect();
    
        random_string
    }

    pub fn qrcode(link: &str, size: u32) -> String {
        let code = QrCode::new(link).unwrap();
        let image = code.render::<Luma<u8>>().max_dimensions(size, size).build();
    
        let mut img_bytes = Vec::new();
        let mut cursor = Cursor::new(&mut img_bytes);
        image.write_to(&mut cursor, image::ImageFormat::Png).unwrap();
    
        Base64::encode(img_bytes)
    }

}
