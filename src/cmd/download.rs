extern crate colored;

use colored::*;

use uuid::Uuid;
use is_url::is_url;

use std::{
    fs::File,
    borrow::Cow,
    error::Error,
    io::{Cursor, Read, Write}
};

use reqwest::{
    Url,
    self
};

use indicatif::{
    ProgressBar,
    ProgressStyle
};

use crate::{
    cmd::syntax::Lexico,
    addons::kindle::Kindle,

    utils::{
        misc::Misc,
        url::UrlMisc,
        file::FileMisc,
        validation::Validate,
    }
};

pub struct Download;

impl Download {

    pub async fn download_and_detect_name(url: &str, path: &str, kindle: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
        Validate::check_url_status(url).await?;
        
        let response = reqwest::get(url).await?;
        
        let total_size = response
            .headers()
            .get(reqwest::header::CONTENT_LENGTH)
            .and_then(|ct_len| ct_len.to_str().ok())
            .and_then(|ct_len| ct_len.parse::<u64>().ok())
            .unwrap_or(0);
    
        let pb = ProgressBar::new(total_size);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .progress_chars("#>-"));
    
        let content_disposition = response.headers().get("content-disposition");
        
        let filename_option = if let Some(value) = content_disposition {
            let cd_string = value.to_str()?;
            let parts: Vec<&str> = cd_string.split("filename=").collect();

            if parts.len() > 1 {
                Some(parts[1].trim_matches('"').to_string())
            } else {
                None
            }
        } else {
            let parsed_url = Url::parse(url)?;
            parsed_url.path_segments()
                .and_then(|segments| segments.last())
                .map(|name| name.to_string())
        };
        
        let filename = filename_option.unwrap_or_else(|| {
            format!("{}.pdf", Uuid::new_v4().to_string())
        });
        
        let _ = Validate::validate_file_type(&filename, ".pdf");

        let output_path = FileMisc::get_output_path(path, &filename);
        let mut dest = File::create(&output_path)?;
        let content = response.bytes().await?;
        let mut reader = Cursor::new(content);
    
        let mut buffer = [0; 8192];
        while let Ok(size) = reader.read(&mut buffer) {
            if size == 0 { break; }
            
            dest.write_all(&buffer[..size])?;
            pb.inc(size as u64);
        }
    
        pb.finish_with_message("Download completed!");
        
        if let Some(kindle_str) = kindle.as_ref() {
            let _ = Kindle::send(&kindle_str, &filename);
        }
    
        Ok(filename)
    }
    
    pub async fn download_file(url: &str, path: &str, no_ignore: bool, no_comments: bool, kindle: Option<String>) -> Result<(), Box<dyn Error>> {
        let mut processed_line: Cow<str> = Cow::Borrowed(
            url.trim()
        );

        let _ = Lexico::handle_comments(&processed_line, no_comments);
        if !is_url(&processed_line) { return Ok(()) }
    
        let result_ignore_macro_flag = Lexico::handle_ignore_macro_flag(&processed_line, no_ignore);
        match result_ignore_macro_flag {
            Ok(new_line) => processed_line = Cow::Owned(new_line),
            Err(_) => return Ok(()),
        }
    
        if let Err(e) = Validate::validate_url(&processed_line) {
            eprintln!("Error: {}", e.to_string().red());
    
            return Err(
                Box::new(e)
            )
        }

        if UrlMisc::is_pdf_file(&processed_line).await? {
            let result = Self::download_and_detect_name(&processed_line, path, kindle).await;
        
            match result {
                Ok(file_name) => {
                    println!("[{}] -> Downloaded file name: {}", Misc::date_time().blue(), file_name.green());
                    return Ok(())
                },
        
                Err(e) => {
                    eprintln!("[{}] -> Error downloading or detecting the name: {}", Misc::date_time().blue(), e.to_string().red());
                    return Err(e);
                }
            }
        }

        Ok(())
    }

}
