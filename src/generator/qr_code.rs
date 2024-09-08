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

pub struct GenQrCode {
    link: String,
    size: u32,
}

impl GenQrCode {

    pub fn new(link: &str, size: usize) -> Self {
        Self {
            link: link.to_string(),
            size: size as u32,
        }
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
