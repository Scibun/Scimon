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
    prime_down::pd_engine::PrimeDownEngine,

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
        reporting::Reporting,
        providers::Providers,
    },

};

pub struct Download;

impl Download {

    async fn make(url: &str, path: &str) -> Result<String, Box<dyn Error>> {
        UrlMisc::check_url_status(url).await?;

        let (request_uri, filename) = Providers::get_from_provider(url).await?;
        let response = reqwest::get(&request_uri).await?;
        let total_size = FileRemote::get_file_size(&request_uri).await?;
    
        let pb = ProgressBar::new(total_size);
        pb.set_style(UI::pb_template());

        pb.set_prefix("Downloading");
    
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
    
        pb.finish_with_message("Download completed!");
        Ok(filename)
    }    

    async fn markdown(url: &str, path: &str) -> Result<(), Box<dyn Error>> {
        let html_content = PrimeDownEngine::render_core(url).await?;
        
        let original_name = UrlMisc::get_last_part(url);
        let new_filename = original_name.replace(".md", ".pdf");
        let output_path = FileMisc::get_output_path(&path, &new_filename);

        PrimeDownEngine::create_pdf(&html_content, output_path, &url).await?;
        SuccessAlerts::download_and_generated_pdf(&new_filename, url);

        Ok(())
    }

    pub async fn pdf(url: &str, path: &str, no_ignore: bool, no_comments: bool) -> Result<(), Box<dyn Error>> {
        let mut line_url: Cow<str> = Cow::Borrowed(
            url.trim()
        );

        Reporting::check_download_errors(&line_url).await?;
        Macros::handle_comments(&line_url, no_comments)?;

        if !is_url(&line_url) {
            return Ok(())
        }
    
        match Macros::handle_ignore_macro_flag(&line_url, no_ignore) {
            Ok(new_line) => line_url = Cow::Owned(new_line),
            Err(_) => return Ok(()),
        }

        if FileRemote::check_content_type(&line_url, "text/markdown").await? {
            Self::markdown(&line_url, &path).await?;
        } else {
            if FileRemote::is_pdf_file(&line_url).await? || Providers::check_provider_domain(&line_url) {
                let result = Self::make(&line_url, path).await;
                
                match result {
                    Ok(file) => SuccessAlerts::download(&file, url),
                    Err(e) => ErrorsAlerts::download(e, url),
                }
            }
        }

        Ok(())
    }

}
