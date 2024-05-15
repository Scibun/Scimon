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
    utils::remote::FileRemote,

    ui::{
        ui_base::UI,
        success_alerts::SuccessAlerts,
    },
};

pub struct PdfCreator;

impl PdfCreator {

    async fn connect_to_browser(content: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let browser = Browser::new(
            LaunchOptionsBuilder::default().build().expect(""),
        )?;

        let pdf_options: Option<PrintToPdfOptions> = None;

        let contents = browser.new_tab()?.navigate_to(
            &format!(
                "data:text/html;base64,{}", BASE64_STANDARD.encode(content)
            )
        )?.wait_until_navigated()?.print_to_pdf(
            pdf_options
        )?;

        Ok(contents)
    }

    pub async fn from_html(content: &str, path: PathBuf, url: &str, file: &str) -> Result<(), Box<dyn Error>> {
        let total_size = FileRemote::get_file_size(url).await?;
        let pdf_contents = Self::connect_to_browser(content).await?;
    
        let pb = ProgressBar::new(total_size);

        pb.set_style(UI::pb_template());
        pb.set_message("Generating PDF...");

        fs::write(path, pdf_contents)?;
        
        pb.finish_with_message("Download and generated completed!");
        SuccessAlerts::download_and_generated_pdf(file, url);

        Ok(())
    }

}
