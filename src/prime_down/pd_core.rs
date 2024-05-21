extern crate colored;

use indicatif::ProgressBar;

use std::{
    fs,
    error::Error,
    path::PathBuf,
};

use pulldown_cmark::{
    html,
    Parser,
    Options,
};

use crate::{
    ui::ui_base::UI,
    utils::remote::FileRemote,
    prime_down::pd_misc::PrimeDownMisc,
};

pub struct PrimeDown;

impl PrimeDown {

    pub async fn render_core(url: &str) -> Result<String, Box<dyn Error>> {
        let markdown_content = FileRemote::content(url).await?;
    
        let options = Options::empty();
        let parser = Parser::new_ext(&markdown_content, options);

        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
    
        Ok(html_output)
    }

    pub async fn create_pdf(content: &str, path: PathBuf, url: &str) -> Result<(), Box<dyn Error>> {
        let len = FileRemote::get_file_size(url).await?;
        let pdf_contents = PrimeDownMisc::connect_to_browser(content).await?;
    
        let pb = ProgressBar::new(len);

        pb.set_style(UI::pb_template());
        pb.set_message("Generating PDF...");

        fs::write(path, pdf_contents)?;
        pb.finish_with_message("Download and generated completed!");

        Ok(())
    }

}
