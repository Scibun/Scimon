use reqwest;

use std::{
    error::Error,
    fs::{self, File},
    io::{BufRead, BufReader}
};

use crate::{
    addons::kindle::Kindle,

    cmd::{
        syntax::Lexico,
        download::Download,
    },

    utils::{
        url::UrlMisc,
        validation::Validate
    }
};

pub struct ReadList;

impl ReadList {

    async fn read_lines<R>(reader: R, no_ignore: bool, no_comments: bool, no_open_link: bool, kindle: Option<String>) -> Result<(), Box<dyn Error>> where R: BufRead {
        let mut path = String::new();

        for line_result in reader.lines() {
            let line = line_result?;
            let trimmed_line = line.trim();
    
            if !Lexico::handle_check_macro_line(&trimmed_line, "open_link") {
                if path.is_empty() {
                    path = Lexico::handle_get_path(trimmed_line);
                    let _ = fs::create_dir(&path);
                }
    
                let url = if !trimmed_line.contains("arxiv.org") {
                    trimmed_line.to_owned()
                } else {
                    trimmed_line.replace("/abs/", "/pdf/")
                };
    
                Download::download_file(
                    &url,
                    &path,
                    no_ignore,
                    no_comments
                ).await?;

                if let Some(kindle_email) = kindle.as_deref() {
                    Kindle::send(&url, &path, kindle_email)?;
                }
            } else {
                if !no_open_link {
                    UrlMisc::open_url(trimmed_line, true);
                }
            }
        }
    
        Ok(())
    }    

    pub async fn read_local_file(run: &str, no_ignore: bool, no_comments: bool, no_open_link: bool, kindle: Option<String>) -> Result<(), Box<dyn Error>> {
        let _ = Validate::validate_file(run).map_err(|e| {
            eprintln!("{}", e);
        });
        
        let file = File::open(run)?;
        let reader = BufReader::new(file);

        Self::read_lines(reader, no_ignore, no_comments, no_open_link, kindle).await?;
        Ok(())
    }
    
    pub async fn read_remote_file(run: &str, no_ignore: bool, no_comments: bool, no_open_link: bool, kindle: Option<String>) -> Result<(), Box<dyn Error>> {
        let _ = Validate::validate_file_type(run, ".txt").map_err(|e| {
            eprintln!("{}", e);
        });
    
        let response = reqwest::get(run).await?;
        let bytes = response.bytes().await?;
        let reader: BufReader<&[u8]> = BufReader::new(&bytes[..]);

        Self::read_lines(reader, no_ignore, no_comments, no_open_link, kindle).await?;
        Ok(())
    }

}
