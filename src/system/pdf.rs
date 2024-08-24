use lopdf::Document;
use indicatif::ProgressBar;
use reqwest::header::CONTENT_TYPE as MimeType;

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
    render::render::Render,
    system::providers::Providers,

    ui::{
        ui_base::UI,
        errors_alerts::ErrorsAlerts,
        success_alerts::SuccessAlerts,
    },

    utils::{
        url::UrlMisc,
        remote::Remote,
        file::FileUtils,
        validation::Validate,
    },
};

pub struct Pdf;

impl Pdf {

    pub async fn is_pdf_file(url: &str) -> Result<bool, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let response = client.get(url).send().await?;

        if !response.status().is_success() {
            return Ok(false);
        }

        if let Some(content_type) = response.headers().get(MimeType) {
            if let Ok(content_type_str) = content_type.to_str() {
                if content_type_str == "application/pdf" {
                    return Ok(true);
                }
            }
        }
    
        Ok(false)
    }

    pub async fn create_pdf(content: &str, path: PathBuf, url: &str) -> Result<(), Box<dyn Error>> {
        let len = Remote::get_file_size(url).await?;
        let pdf_contents = Render::connect_to_browser(content).await?;
    
        let pb = ProgressBar::new(len);

        pb.set_style(UI::pb_template());
        pb.set_message("Generating PDF...");

        fs::write(path, pdf_contents)?;
        pb.finish_with_message("Download and generated completed!");

        Ok(())
    }

    pub async fn download(url: &str, path: &str) -> Result<String, Box<dyn Error>> {
        UrlMisc::check_url_status(url).await?;

        let (request_uri, filename) = Providers::new(url).get_from_provider().await?;
        let response = reqwest::get(&request_uri).await?;
        let total_size = Remote::get_file_size(&request_uri).await?;
    
        let pb = ProgressBar::new(total_size);
        pb.set_style(UI::pb_template());
    
        let output_path = FileUtils::get_output_path(path, &filename);
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

    pub async fn download_line(line_url: &str, url: &str, path: &str) -> Result<String, Box<dyn Error>> {
        if Self::is_pdf_file(&line_url).await? || Providers::new(url).check_provider_domain() && !line_url.contains(".md") {
            let result = Self::download(&line_url, path).await;
            
            match result {
                Ok(file) => {
                    let file_path = &format!("{}{}", &path, &file);
                    let password = Self::is_pdf_encrypted(&file_path);
                    
                    SuccessAlerts::download(&file, url, password);
                    return Ok(file_path.to_string())
                },

                Err(e) => ErrorsAlerts::download(e, url),
            }
        }

        Ok("".to_string())
    }

}
