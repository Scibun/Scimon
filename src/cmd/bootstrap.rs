use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};

use reqwest;

use crate::utils::validation::Validate;

use crate::cmd::download::Download;

pub struct Bootstrap;

impl Bootstrap {

    pub async fn read_paimon_local_file(file_path: &str, no_ignore: bool, no_comments: bool, kindle: Option<String>) -> Result<(), Box<dyn Error>> {
        if let Err(e) = Validate::validate_file(file_path) {
            eprintln!("{}", e);
            return Err(Box::new(e));
        }
    
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
    
        for line_result in reader.lines() {
            let line = line_result?;
            let trimmed_line = line.trim();
            let _ = Download::run_download_current_line(&trimmed_line, no_ignore, no_comments, kindle.clone()).await?;
        }
    
        Ok(())
    }
    
    pub async fn read_paimon_remote_file(url: &str, no_ignore: bool, no_comments: bool, kindle: Option<String>) -> Result<(), Box<dyn Error>> {
        Validate::validate_file_type(url, ".txt")?;
    
        let response = reqwest::get(url).await?;
        let bytes = response.bytes().await?;
    
        let reader = BufReader::new(&bytes[..]);
    
        for line_result in reader.lines() {
            let line = line_result?;
            let trimmed_line = line.trim();
            let _ = Download::run_download_current_line(&trimmed_line, no_ignore, no_comments, kindle.clone()).await?;
        }
    
        Ok(())
    }
    
    pub async fn read_paimon_file(run: &str, no_ignore: bool, no_comments: bool, kindle: Option<String>) -> Result<(), Box<dyn Error>> {
        if !run.starts_with("http") {
            if let Err(_) = Self::read_paimon_local_file(run, no_ignore, no_comments, kindle).await {}
        } else {
            if let Err(e) = Self::read_paimon_remote_file(run, no_ignore, no_comments, kindle).await {
                eprintln!("Error: {}", e);
            }
        }
        Ok(())
    }
    
}