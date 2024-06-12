use regex::Regex;

use std::{
    fs,
    error::Error,
};

use crate::{
    args_cli::Flags,
    system::markdown::Markdown,
    monset::vars::Vars,
    prime_down::pd_core::PrimeDown,
    ui::macros_alerts::MacrosAlerts,
    regexp::regex_blocks::BlocksRegExp,

    utils::{
        url::UrlMisc,
        file::FileMisc,
        remote::FileRemote,
    }
};

pub struct ReadMeBlock;

impl ReadMeBlock {

    fn render(input: &str) -> Option<String> {
        let contents = fs::read_to_string(input).expect("Failed to read file");
    
        let start_pattern = Regex::new(BlocksRegExp::GET_README_BLOCK[0]).unwrap();
    
        if let Some(start_match) = start_pattern.find(&contents) {
            let start_index = start_match.end();
            let mut end_index = start_index;
            let mut open_braces = 1;
    
            for (i, c) in contents[start_index..].char_indices() {
                match c {
                    '{' => open_braces += 1,
                    '}' => open_braces -= 1,
                    _ => {}
                }
    
                if open_braces == 0 {
                    end_index = start_index + i;
                    break;
                }
            }
    
            if open_braces != 0 {
                eprintln!("Unmatched braces in the 'readme' block.");
                return None;
            }
    
            let markdown_block = &contents[start_index..end_index].trim();
            let unindented_markdown_block = markdown_block.lines()
                .map(|line| line.trim_start())
                .collect::<Vec<&str>>()
                .join("\n");

            let output = Markdown::append_extras_and_render(&unindented_markdown_block);
            Some(output)
        } else {
            None
        }
    }

    pub async fn render_block_and_save_file(run: &str, flags: &Flags) {
        if !flags.no_readme {
            if let Some(markdown_html) = Self::render(run) {
                let path = Markdown::get_filename_rendered(run);
                
                if let Ok(contents) = PrimeDown::render_content(&run, markdown_html).await {
                    FileMisc::write_file(&path, contents);
                    Markdown::open_file(&path, flags.no_open_link);
                    
                    MacrosAlerts::readme(&path);
                }
            }
        }
    }

    pub async fn render_var_and_save_file(contents: &str, flags: &Flags) -> Result<(), Box<dyn Error>> {
        if !flags.no_readme {
            if let Some(url) = Vars::get_readme(contents).await {
                let get_last_part = &UrlMisc::get_last_part(&url);
    
                let path = Markdown::get_filename_rendered(
                    &get_last_part.replace(".md", ".html")
                );
    
                let markdown_content = FileRemote::content(&url).await?;
                let contents_extras = Markdown::append_extras_and_render(&markdown_content);

                if let Ok(contents) = PrimeDown::render_content(&get_last_part, contents_extras).await {
                    FileMisc::write_file(&path, contents);
                    Markdown::open_file(&path, flags.no_open_link);
                    
                    MacrosAlerts::readme(&path);
                }
            }
        }
    
        Ok(())
    }

}
