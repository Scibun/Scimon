extern crate open;

use regex::Regex;

use std::{
    fs,
    env,
    error::Error,
};

use pulldown_cmark::{
    html,
    Parser,
    Options,
};

use crate::{
    configs::settings::Settings,
    syntax::vars_block::VarsBlock,
    ui::macros_alerts::MacrosAlerts,
    regexp::regex_blocks::BlocksRegExp,

    prime_down::{
        pd_io::PrimeDownIO,
        pd_misc::PrimeDownMisc,
        pd_extras::PrimeDownExtras,
    },

    utils::{
        url::UrlMisc,
        file::FileMisc,
        remote::FileRemote,
        generate::Generate,
    }
};

pub struct ReadMeBlock;

impl ReadMeBlock {

    fn get_readme_filename(file: &str) -> String {
        let filename = if Settings::get("render_markdown.overwrite", "BOOLEAN") == true {
            ".html".to_string()
        } else {
            format!(
                "_{}.html", Generate::random_string(16)
            )
        };

        PrimeDownIO::get_file_path(file).replace(
            ".html", &filename
        )
    }

    fn append_extras_and_render(markdown: &str) -> String {
        let markdown_block_extras_qrcode = PrimeDownExtras::qrcode(&markdown);
        let markdown_block_extras_gist = PrimeDownExtras::gist(&markdown_block_extras_qrcode);

        let parser = Parser::new_ext(&markdown_block_extras_gist, Options::all());
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        format!("<div class=\"markdown-content\">{}</div>", html_output)
    }

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

            let output = Self::append_extras_and_render(&unindented_markdown_block);
            Some(output)
        } else {
            None
        }
    }
    
    fn open_readme_url(path: &str, no_open_link: bool) {
        if !no_open_link {
            let full_path = env::current_dir().expect(
                ""
            ).join(&path).to_str().unwrap().replace(
                "\\", "/"
            );

            let url_file = &format!(
                "file://{}", full_path
            );

            let _ = open::that(&url_file);
        }
    }

    pub fn render_block_and_save_file(file: &str, no_open_link: bool, no_readme: bool) {
        if !no_readme {
            if let Some(markdown_html) = Self::render(file) {
                let path = Self::get_readme_filename(file);
                let contents = PrimeDownMisc::render_content(&file, markdown_html);

                FileMisc::write_file(&path, contents);
                Self::open_readme_url(&path, no_open_link);
                
                MacrosAlerts::readme(&path);
            }
        }
    }

    pub async fn render_var_and_save_file(contents: &str, no_open_link: bool) -> Result<(), Box<dyn Error>> {
        if let Some(url) = VarsBlock::get_readme(contents).await {
            let get_last_part = &UrlMisc::get_last_part(&url);

            let path = Self::get_readme_filename(
                &get_last_part.replace(".md", ".html")
            );

            let markdown_content = FileRemote::content(&url).await?;
            let contents_extras = Self::append_extras_and_render(&markdown_content);
            let contents = PrimeDownMisc::render_content(&get_last_part, contents_extras);
        
            FileMisc::write_file(&path, contents);
            Self::open_readme_url(&path, no_open_link);
            
            MacrosAlerts::readme(&path);
        }
    
        Ok(())
    }

}
