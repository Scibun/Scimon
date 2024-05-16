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
    ui::ui_base::UI,

    utils::{
        base64::Base64,
        remote::FileRemote,
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
            &Base64::encode_html(content)
        )?.wait_until_navigated()?.print_to_pdf(
            pdf_options
        )?;

        Ok(contents)
    }

    pub async fn from_html(content: &str, path: PathBuf, url: &str) -> Result<(), Box<dyn Error>> {
        let len = FileRemote::get_file_size(url).await?;
        let pdf_contents = Self::connect_to_browser(content).await?;
    
        let pb = ProgressBar::new(len);

        pb.set_style(UI::pb_template());
        pb.set_message("Generating PDF...");

        fs::write(path, pdf_contents)?;
        pb.finish_with_message("Download and generated completed!");

        Ok(())
    }

}
