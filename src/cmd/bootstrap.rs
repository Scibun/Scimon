use std::{
    fs::File,
    error::Error,
    io::{BufReader, BufRead}
};

use reqwest;

extern crate colored;

use crate::utils::{
    misc::Misc,
    file::FileUtils,
    validation::Validate
};

use super::syntax::Lexico;

use crate::cmd::download::Download;

pub struct Paimon;

impl Paimon {

    pub async fn read_local_file(file_path: &str, no_ignore: bool, no_comments: bool, kindle: Option<String>) -> Result<(), Box<dyn Error>> {
        if let Err(e) = Validate::validate_file(file_path) {
            eprintln!("{}", e);
            return Err(Box::new(e));
        }
        
        let mut path = String::new();
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
    
        for line_result in reader.lines() {
            let line = line_result?;
            let trimmed_line = line.trim();

            if !Lexico::handle_check_macro_line(&trimmed_line, "open_link") {
                if path.is_empty() {
                    path = Lexico::handle_get_path(trimmed_line);
                    let _ = FileUtils::new_path(&path);
                }

                Download::download_file(
                    &trimmed_line,
                    &path,
                    no_ignore, 
                    no_comments, 
                    kindle.clone()
                ).await?;
            } else {
                let _ = Misc::open_url(trimmed_line);
            }
        }
    
        Ok(())
    }
    
    pub async fn read_remote_file(url: &str, no_ignore: bool, no_comments: bool, kindle: Option<String>) -> Result<(), Box<dyn Error>> {
        Validate::validate_file_type(url, ".txt")?;
    
        let response = reqwest::get(url).await?;
        let bytes = response.bytes().await?;
    
        let reader = BufReader::new(&bytes[..]);
    
        for line_result in reader.lines() {
            let line = line_result?;
            let trimmed_line = line.trim();
            
            let path = Lexico::handle_get_path(trimmed_line);
            let _ = FileUtils::new_path(&path);

            Download::download_file(
                &trimmed_line,
                &path,
                no_ignore, 
                no_comments, 
                kindle.clone()
            ).await?;
        }
    
        Ok(())
    }
    
    pub async fn run(run: &str, no_ignore: bool, no_comments: bool, kindle: Option<String>) -> Result<(), Box<dyn Error>> {
        if !run.starts_with("http") {
            if let Err(_) = Self::read_local_file(run, no_ignore, no_comments, kindle).await {}
        } else {
            if let Err(e) = Self::read_remote_file(run, no_ignore, no_comments, kindle).await {
                eprintln!("Error: {}", e);
            }
        }

        Ok(())
    }

}
