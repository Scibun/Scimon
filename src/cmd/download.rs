use reqwest;
use is_url::is_url;
use indicatif::ProgressBar;

use std::{
    fs::File,
    borrow::Cow,
    error::Error,

    io::{
        Read,
        Write,
        Cursor,
    }
};

use crate::{
    ui::{
        ui_base::UI,
        errors_alerts::ErrorsAlerts,
        success_alerts::SuccessAlerts,
    },
    
    utils::{
        url::UrlMisc,
        file::FileMisc,
        remote::FileRemote,
        validation::Validate,
    },

    system::{
        syntax::Macros,
        markdown::Markdown,
        reporting::Reporting,
        providers::Providers,
        pdf_creator::PdfCreator,
    },

};

pub struct Download;

impl Download {

    async fn make_download(url: &str, path: &str) -> Result<String, Box<dyn Error>> {
        Validate::check_url_status(url).await?;

        let (request_uri, filename) = Providers::get_from_provider(url).await?;
        let response = reqwest::get(&request_uri).await?;    
        let total_size = FileRemote::get_file_size(&request_uri).await?;
    
        let pb = ProgressBar::new(total_size);
        pb.set_style(UI::pb_template());

        pb.set_prefix("Downloading");
    
        let output_path = FileMisc::get_output_path(path, &filename);
        let _ = Validate::validate_file_type(&filename, ".pdf");
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
        Ok(filename)
    }    

    pub async fn download_markdown(url: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let html_content = Markdown::render_core(url).await?;
        
        let original_name = UrlMisc::get_last_part(url);
        let new_filename = original_name.replace(".md", ".pdf");
        let output_path = FileMisc::get_output_path(&path, &new_filename);

        PdfCreator::from_html(&html_content, output_path, &url, &new_filename).await?;
        Ok(())
    }

    pub async fn download_pdf(url: &str, path: &str, no_ignore: bool, no_comments: bool) -> Result<(), Box<dyn Error>> {
        let mut processed_line: Cow<str> = Cow::Borrowed(
            url.trim()
        );

        Reporting::check_download_errors(&processed_line).await?;
        let _ = Macros::handle_comments(&processed_line, no_comments);

        if !is_url(&processed_line) {
            return Ok(())
        }
    
        let result_ignore_macro_flag = Macros::handle_ignore_macro_flag(&processed_line, no_ignore);
        match result_ignore_macro_flag {
            Ok(new_line) => processed_line = Cow::Owned(new_line),
            Err(_) => return Ok(()),
        }
    
        if let Err(e) = Validate::validate_url(&processed_line) {
            ErrorsAlerts::generic(&e.to_string());
    
            return Err(
                Box::new(e)
            )
        }

        if processed_line.ends_with(".md") {
            Self::download_markdown(&processed_line, &path).await?;
        } else {
            if FileRemote::is_pdf_file(&processed_line).await? || Providers::check_provider_domain(&processed_line) {
                let result = Self::make_download(&processed_line, path).await;
                
                match result {
                    Ok(file) => SuccessAlerts::download(&file, url),
                    Err(e) => ErrorsAlerts::download(e, url),
                }
            }
        }

        Ok(())
    }

}
