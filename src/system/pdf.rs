use lopdf::Document;
use indicatif::ProgressBar;

use std::{
    error::Error,
    path::PathBuf,

    io::{
        Read,
        Write,
        Cursor,
        BufReader
    },
    
    fs::{
        self,
        File
    },
};

use crate::{
    ui::ui_base::UI,
    system::providers::Providers,
    prime_down::pd_core::PrimeDown,

    utils::{
        url::UrlMisc,
        file::FileMisc,
        remote::FileRemote,
        validation::Validate,
    },
};

pub struct Pdf;

impl Pdf {

    pub async fn is_pdf_file(url: &str) -> Result<bool, Box<dyn Error>> {
        let client: reqwest::Client = reqwest::Client::new();
        let response = client.get(url).send().await?;

        if !response.status().is_success() {
            return Ok(false);
        }

        if let Some(content_type) = response.headers().get(reqwest::header::CONTENT_TYPE) {
            if let Ok(content_type_str) = content_type.to_str() {
                if content_type_str == "application/pdf" {
                    return Ok(true);
                }
            }
        }
    
        Ok(false)
    }

    pub async fn create_pdf(content: &str, path: PathBuf, url: &str) -> Result<(), Box<dyn Error>> {
        let len = FileRemote::get_file_size(url).await?;
        let pdf_contents = PrimeDown::connect_to_browser(content).await?;
    
        let pb = ProgressBar::new(len);

        pb.set_style(UI::pb_template());
        pb.set_message("Generating PDF...");

        fs::write(path, pdf_contents)?;
        pb.finish_with_message("Download and generated completed!");

        Ok(())
    }

    pub async fn download(url: &str, path: &str) -> Result<String, Box<dyn Error>> {
        UrlMisc::check_url_status(url).await?;

        let (request_uri, filename) = Providers::get_from_provider(url).await?;
        let response = reqwest::get(&request_uri).await?;
        let total_size = FileRemote::get_file_size(&request_uri).await?;
    
        let pb = ProgressBar::new(total_size);
        pb.set_style(UI::pb_template());
    
        let output_path = FileMisc::get_output_path(path, &filename);
        let mut dest = File::create(&output_path)?;
        let content = response.bytes().await?;
        let mut reader = Cursor::new(content);

        let _ = Validate::file_type(&filename, ".pdf");
    
        let mut buffer = [0; 8192];
        while let Ok(size) = reader.read(&mut buffer) {
            if size == 0 { break; }
            
            dest.write_all(&buffer[..size])?;
            pb.inc(size as u64);
        }
    
        Ok(filename)
    }

    pub fn is_pdf_encrypted(file_path: &str) -> bool {
        if let Ok(file) = File::open(file_path) {
            let reader = BufReader::new(file);

            if let Ok(doc) = Document::load_from(reader) {
                return doc.is_encrypted();
            }
        }
        
        false
    }

}
