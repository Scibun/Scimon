pub mod file {

    extern crate colored;

    use colored::*;

    use reqwest;
    use uuid::Uuid;
    use is_url::is_url;
    use std::borrow::Cow;
    use std::error::Error;

    use reqwest::Url;
    use std::fs::File;
    use std::io::{Read, Write, Cursor};
    use indicatif::{ProgressBar, ProgressStyle};
    
    use crate::cmd::validation::data::{
        validate_url,
        check_url_status,
        validate_file_type
    };

    use crate::cmd::syntax::macros_flags::{
        handle_comments,
        handle_ignore_macro_flag
    };
    
    use crate::cmd::kindle::send::send_kindle;

    async fn download_and_detect_name(url: &str, kindle: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
        check_url_status(url).await?;
        
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
        
        let _ = validate_file_type(&filename, ".pdf");

        let mut dest = File::create(&filename)?;
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
            let _ = send_kindle(&kindle_str, &filename);
        }

        Ok(filename)
    }
    
    pub async fn run_download_current_line(line: &str, no_ignore: bool, kindle: Option<String>) -> Result<(), Box<dyn Error>> {
        let mut processed_line: Cow<str> = Cow::Borrowed(
            line.trim()
        );

        let _ = handle_comments(&processed_line);
        if !is_url(&processed_line) { return Ok(()); }

        let result_ignore_macro_flag = handle_ignore_macro_flag(&processed_line, no_ignore);
        match result_ignore_macro_flag {
            Ok(new_line) => processed_line = Cow::Owned(new_line),
            Err(_) => return Ok(()),
        }

        if let Err(e) = validate_url(&processed_line) {
            eprintln!("{}", e);

            return Err(
                Box::new(e)
            )
        }

        let result = download_and_detect_name(&processed_line, kindle).await;

        match result {
            Ok(file_name) => {
                println!("-> Downloaded file name: {}", file_name.green());
                return Ok(())
            },

            Err(e) => {
                eprintln!("-> Error downloading or detecting the name: {}", e);
                return Err(e);
            }
        }
    }

}
