use std::fs;
use std::path::Path;

use std::fs::File;
use std::io::{Read, Write};
use libflate::zlib::Encoder;
use std::io::Result as IoResult;
use zip::{ZipWriter, CompressionMethod, write::FileOptions};

pub struct FileUtils;

impl FileUtils {
    
    pub fn is_file_over(path: &str, size: u32) -> bool {
        let path = Path::new(path);
    
        if path.is_file() {
            if let Ok(metadata) = fs::metadata(path) {
                let file_size = metadata.len();
                file_size > (size * 1024 * 1024) as u64
            } else {
                false
            }
        } else {
            false
        }
    }
    
    pub fn get_file_name_string(file_path: &str) -> String {
        let path = Path::new(file_path);
        
        path.file_name()
            .and_then(|name| name.to_str())
            .map(|s| s.to_owned())
            .unwrap_or_else(|| "unknown_filename".to_owned())
    }
    
    pub fn get_file_name(path: &str) -> Result<String, &'static str> {
        Path::new(path)
            .file_name()
            .and_then(|name| name.to_str())
            .map(|s| s.to_string())
            .ok_or("Failed to get the file name.")
    }
    
    pub fn check_file_exists(path: &str) -> Result<(), &'static str> {
        let metadata = fs::metadata(
            Path::new(path)
        );
        
        if metadata.is_ok() && metadata.unwrap().is_file() {
            Ok(())
        } else {
            Err("The specified file does not exist.")
        }
    }
    
    pub fn compress_pdf_to_zip(pdf_file: &str, zip_file_name: &str) -> IoResult<String> {
        let mut pdf_data = Vec::new();
        File::open(pdf_file)?.read_to_end(&mut pdf_data)?;
    
        let zip_file = File::create(zip_file_name)?;
        let mut zip = ZipWriter::new(zip_file);
    
        let options = FileOptions::default()
            .compression_method(zip::CompressionMethod::Stored)
            .unix_permissions(0o755)
            .compression_method(CompressionMethod::Deflated);
    
        let pdf_file_name = Self::get_file_name(pdf_file).unwrap_or_else(|err| {
            println!("{}", err);
            "".to_string()
        });
    
        zip.start_file(&pdf_file_name, options)?;
        let encoder = Encoder::new(Vec::new())?;
        let mut compressed_pdf = encoder.into_inner();
        compressed_pdf.write_all(&pdf_data)?;
        zip.write_all(&compressed_pdf)?;
        zip.finish()?;
    
        let output_file_name = Self::get_file_name(zip_file_name).unwrap_or_else(|err| {
            println!("{}", err);
            "".to_string()
        });
    
        Ok(output_file_name)
    }

}
