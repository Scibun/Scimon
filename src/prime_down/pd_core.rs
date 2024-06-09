use minify::html::minify;

use std::error::Error;

use headless_chrome::{
    Browser, 
    LaunchOptionsBuilder,
    types::PrintToPdfOptions,
};

use crate::{
    utils::base64::Base64,
    utils::remote::FileRemote,
    configs::settings::Settings,
    consts::prime_down::PrimeDownEnv,
    prime_down::inject::pd_inject::PrimeDownInject,
};

pub struct PrimeDown;

impl PrimeDown {

    pub async fn render_content(file: &str, md_content: String) -> Result<String, Box<dyn Error>> {
        let minify_prop = Settings::get("render_markdown.minify_html", "BOOLEAN");
        let template_content = FileRemote::content(PrimeDownEnv::README_TEMPLATE_LINK).await?;
        let content = PrimeDownInject::content(&file, template_content, md_content);

        let output = if minify_prop == true {
            minify(&content)
        } else {
            content
        };

        Ok(output)
    }

    pub async fn connect_to_browser(content: &str) -> Result<Vec<u8>, Box<dyn Error>> {
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

}
