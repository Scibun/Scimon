use image::Luma;
use qrcode::QrCode;
use std::io::Cursor;

use crate::utils::base64::Base64;

pub struct GenQrCode {
    link: String,
    size: u32,
}

impl GenQrCode {

    pub fn new(link: &str, size: u32) -> Self {
        Self {
            link: link.to_string(),
            size,
        }
    }

    pub fn get(&self) -> String {
        let code = QrCode::new(self.link.as_str()).unwrap();
        let image = code.render::<Luma<u8>>().max_dimensions(self.size, self.size).build();
    
        let mut img_bytes = Vec::new();
        let mut cursor = Cursor::new(&mut img_bytes);
        image.write_to(&mut cursor, image::ImageFormat::Png).unwrap();
    
        let qr_code = Base64::encode(img_bytes);
        format!("data:image/png;base64,{}", qr_code)
    }

    pub fn html(&self) -> String {
        let qr_code_base64 = self.get();

        format!(
            "<p class='qrcode'>
                <img src='{}' alt='QR Code of {}' />
            </p>", qr_code_base64, self.link
        )
    }

}
