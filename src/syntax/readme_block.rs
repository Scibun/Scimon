use regex::Regex;

use std::{
    fs,
    env
};

use pulldown_cmark::{
    html,
    Parser,
    Options,
};

use crate::{
    configs::settings::Settings,
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
        generate::Generate,
    }
};

pub struct ReadMeBlock;

impl ReadMeBlock {

    fn render(input: &str) -> Option<String> {
        let contents = fs::read_to_string(input).expect("Failed to read file");
    
        let start_pattern = Regex::new(BlocksRegExp::GET_README[0]).unwrap();
    
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

            let markdown_block_extras_qrcode = PrimeDownExtras::qrcode(&unindented_markdown_block);
            let markdown_block_extras_gist = PrimeDownExtras::gist(&markdown_block_extras_qrcode);
    
            let parser = Parser::new_ext(&markdown_block_extras_gist, Options::all());
            let mut html_output = String::new();
            html::push_html(&mut html_output, parser);
    
            Some(
                format!("<div class=\"markdown-content\">{}</div>", html_output)
            )
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

            UrlMisc::open_url(&url_file, false);
        }
    }

    pub fn render_and_save_file(file: &str, no_open_link: bool, no_readme: bool) {
        if !no_readme {
            if let Some(markdown_html) = Self::render(file) {
                let filename = if Settings::get("render_markdown.overwrite", "BOOLEAN") == true {
                    ".html".to_string()
                } else {
                    format!(
                        "_{}.html", Generate::random_string(16)
                    )
                };

                let path = PrimeDownIO::get_file_path(file).replace(
                    ".html", &filename
                );

                let contents = PrimeDownMisc::render_content(&file, markdown_html);

                FileMisc::write_file(&path, contents);
                Self::open_readme_url(&path, no_open_link);
                
                MacrosAlerts::readme(&path);
            }
        }
    }

}
