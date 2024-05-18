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
    regex::regex_macros::MacrosRegExp,

    prime_down::{
        pd_misc::PrimeDownMisc,
        pd_extras::PrimeDownExtras,
    },
};

pub struct PrimeDown;

impl PrimeDown {
    
    pub fn render_readme(file: &str) -> Option<String> {
        let contents = fs::read_to_string(&file).expect("");
        let (start_regex, end_regex) = PrimeDownMisc::start_end_macros_position().unwrap();

        if start_regex.is_match(&contents) && end_regex.is_match(&contents) {
            let start_match = start_regex.find(&contents).unwrap();
            let end_match = end_regex.find(&contents).unwrap();
        
            let start_index = start_match.start();
            let end_index = end_match.start() + MacrosRegExp::GET_README[1].len();
        
            let markdown_block = &contents[start_index..end_index];
            let markdown_block_extras_qrcode = &PrimeDownExtras::qrcode(markdown_block);
            let markdown_block_extras_gist = &PrimeDownExtras::gist(markdown_block_extras_qrcode);
            let parser = Parser::new_ext(&markdown_block_extras_gist, Options::all());
        
            let mut html_output = String::new();
            html::push_html(&mut html_output, parser);
        
            Some(html_output)
        } else {
            None
        }
    }

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
