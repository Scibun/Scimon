use qrcode::QrCode;

use std::{
    fs::write,
    io::Cursor,
    path::Path,
    error::Error,
};

use image::{
    Luma,
    ImageFormat,
};

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
        image.write_to(&mut cursor, ImageFormat::Png).unwrap();
    
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

    pub fn png(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let code = QrCode::new(self.link.as_str())?;
        let image = code.render::<Luma<u8>>().max_dimensions(
            self.size, self.size
        ).build();
        
        let file_path = Path::new(file_path);
        let mut cursor = Cursor::new(Vec::new());
        image.write_to(&mut cursor, ImageFormat::Png)?;
        
        write(file_path, cursor.into_inner())?;        
        Ok(())
    }

}
