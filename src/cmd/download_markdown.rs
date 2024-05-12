use reqwest;
use base64::prelude::*;

use std::{
    fs,
    error::Error,
    path::PathBuf,
};

use headless_chrome::{
    Browser, 
    LaunchOptionsBuilder,
    types::PrintToPdfOptions,
};

use pulldown_cmark::{
    html,
    Parser,
    Options,
};

use indicatif::{
    ProgressBar,
    ProgressStyle
};

use crate::{
    configs::global::Global,
    ui::ui_alerts::PaimonUIAlerts,

    utils::{
        url::UrlMisc,
        file::FileMisc,
    }
};

pub struct DownloadMarkdown;

impl DownloadMarkdown {
    
    async fn fetch_markdown_content_size(url: &str) -> Result<u64, Box<dyn Error>> {
        let response = reqwest::get(url).await?;
    
        let total_size = response
            .headers()
            .get(reqwest::header::CONTENT_LENGTH)
            .and_then(|ct_len| ct_len.to_str().ok())
            .and_then(|ct_len| ct_len.parse::<u64>().ok())
            .unwrap_or(0);

        Ok(total_size)
    }

    async fn fetch_markdown_content(url: &str) -> Result<String, Box<dyn Error>> {
        let response = reqwest::get(url).await?;
        let content = response.text().await?;
        Ok(content)
    }

    async fn markdown_to_html(url: &str) -> Result<String, Box<dyn Error>> {
        let markdown_content = Self::fetch_markdown_content(url).await?;
    
        let options = Options::empty();
        let parser = Parser::new_ext(&markdown_content, options);

        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
    
        Ok(html_output)
    }

    async fn html_to_pdf(content: &str, path: PathBuf, url: &str, file: &str) -> Result<(), Box<dyn Error>> {
        let total_size = Self::fetch_markdown_content_size(url).await?;

        let browser = Browser::new(
            LaunchOptionsBuilder::default().build().expect(""),
        )?;

        let tab = browser.new_tab()?;

        let html_str = BASE64_STANDARD.encode(content);
        let file_path = format!("data:text/html;base64,{}", html_str);

        let pdf_options: Option<PrintToPdfOptions> = None;
        let contents = tab.navigate_to(
            &file_path
        )?.wait_until_navigated()?.print_to_pdf(
            pdf_options
        )?;
    
        let pb = ProgressBar::new(total_size);
        pb.set_style(
            ProgressStyle::with_template(Global::PB_STYLE).unwrap().progress_chars("█░")
        );

        pb.set_message("Generating PDF...");

        fs::write(path, contents)?;
        
        pb.finish_with_message("Download and generated completed!");
        PaimonUIAlerts::success_download_and_generated_pdf(file, url);

        Ok(())
    }

    pub async fn generate_pdf(url: &str, path: &str) -> Result<String, Box<dyn std::error::Error>> {
        let html_content = Self::markdown_to_html(url).await?;
        
        let original_name = UrlMisc::get_last_part(url);
        let new_filename = original_name.replace(".md", ".pdf");

        let output_path = FileMisc::get_output_path(&path, &new_filename);

        Self::html_to_pdf(&html_content, output_path, &url, &new_filename).await?;
        Ok(new_filename.to_string())
    }

}

