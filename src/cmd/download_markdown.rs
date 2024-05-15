use base64::prelude::*;
use indicatif::ProgressBar;

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

use crate::{
    system::markdown::Markdown,

    ui::{
        ui_base::UI,
        success_alerts::SuccessAlerts,
    },

    utils::{
        url::UrlMisc,
        file::FileMisc,
        remote::FileRemote,
    }
};

pub struct DownloadMarkdown;

impl DownloadMarkdown {

    async fn html_to_pdf(content: &str, path: PathBuf, url: &str, file: &str) -> Result<(), Box<dyn Error>> {
        let total_size = FileRemote::get_file_size(url).await?;

        let browser = Browser::new(
            LaunchOptionsBuilder::default().build().expect(""),
        )?;

        let tab = browser.new_tab()?;

        let html_str = BASE64_STANDARD.encode(content);
        let file_path = format!("data:text/html;base64,{}", html_str);

        let pdf_options: Option<PrintToPdfOptions> = None;

        let contents = tab.navigate_to(&file_path)?.wait_until_navigated()?.print_to_pdf(pdf_options)?;
    
        let pb = ProgressBar::new(total_size);

        pb.set_style(UI::pb_template());

        pb.set_message("Generating PDF...");

        fs::write(path, contents)?;
        
        pb.finish_with_message("Download and generated completed!");
        SuccessAlerts::download_and_generated_pdf(file, url);

        Ok(())
    }

    pub async fn generate_pdf(url: &str, path: &str) -> Result<String, Box<dyn std::error::Error>> {
        let html_content = Markdown::render_core(url).await?;
        
        let original_name = UrlMisc::get_last_part(url);
        let new_filename = original_name.replace(".md", ".pdf");

        let output_path = FileMisc::get_output_path(&path, &new_filename);

        Self::html_to_pdf(&html_content, output_path, &url, &new_filename).await?;
        Ok(new_filename.to_string())
    }

}
