use std::{
    fs::File,
    error::Error,

    io::{
        Read,
        BufRead,
        BufReader,
    },
};

use sha2::{
    Digest,
    Sha256,
};

use crate::utils::remote::FileRemote;

pub struct Hashes;

impl Hashes {

    pub fn read_local_file(file: &str) -> Result<(Vec<String>, usize), Box<dyn Error>> {
        let mut lines = Vec::new();

        let file = File::open(file)?;
        let reader = BufReader::new(file);
    
        for line in reader.lines() {
            let line = line?;

            if !line.trim().is_empty() {
                lines.push(line);
            }
        }
    
        let total_lines = &lines.len();
        Ok((lines, *total_lines))
    }
    
    pub async fn read_remote_file(url: &str) -> Result<(Vec<String>, usize), Box<dyn Error>> {
        let body = FileRemote::content(url).await?;

        let lines: Vec<String> = body
            .lines()
            .map(|s| s.to_string())
            .filter(|line| !line.trim().is_empty())
            .collect();

        let total_lines = &lines.len();
        Ok((lines, *total_lines))
    }

    pub fn calculate_local_sha256(file_path: &str) -> Result<String, Box<dyn Error>> {
        let mut file = File::open(file_path)?;
        let mut hasher = Sha256::new();

        let mut buffer = [0; 1024];
        
        loop {
            let bytes_read = file.read(&mut buffer)?;

            if bytes_read == 0 { break; }

            hasher.update(&buffer[..bytes_read]);
        }
    
        let hash = hasher.finalize();
        Ok(format!("{:x}", hash))
    }

}
