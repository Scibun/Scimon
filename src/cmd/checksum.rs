use regex::Regex;

use std::{
    error::Error,

    io::{
        Read,
        Write,
        BufRead,
        BufReader,
    },
    
    fs::{
        File,
        self,
    },
};

use sha2::{
    Digest,
    Sha256,
};

use crate::{
    syntax::vars_block::VarsBlock,
    regexp::regex_core::CoreRegExp,

    ui::{
        ui_base::UI,
        checksum_alerts::ChecksumAlerts, 
    },

    utils::{
        file::FileMisc,
        remote::FileRemote,
    },
};

pub struct Checksum;

impl Checksum {

    fn read_local_file(file: &str) -> Result<(Vec<String>, usize), Box<dyn Error>> {
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
    
    async fn read_remote_file(url: &str) -> Result<(Vec<String>, usize), Box<dyn Error>> {
        let body = FileRemote::content(url, false).await?;

        let lines: Vec<String> = body
            .lines()
            .map(|s| s.to_string())
            .filter(|line| !line.trim().is_empty())
            .collect();

        let total_lines = &lines.len();
        Ok((lines, *total_lines))
    }
    
    pub fn extract_hashes_and_filenames(line: &str) -> Result<(String, String), Box<dyn Error>> {
        let re = Regex::new(CoreRegExp::GET_CHECKSUM).unwrap();
        let captures = re.captures(line).ok_or("No match found")?;

        let hash = captures.get(1).unwrap().as_str().to_string();
        let filename = captures.get(2).unwrap().as_str().to_string();

        Ok((hash, filename))
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

    pub async fn generate_hashes(path: &str, file: &str, no_checksum: bool) -> Result<(), Box<dyn Error>> {
        if !no_checksum {
            let path_file = format!(
                "{}{}", path, FileMisc::replace_extension(file, "sha256")
            );
    
            let mut output_file = File::create(&path_file)?;
    
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let item = entry.path();
                let file_path = item.to_string_lossy().to_string();
        
                if item.is_file() && !file_path.ends_with(".sha256") {
                    let hash = Self::calculate_local_sha256(&file_path)?;
    
                    writeln!(
                        output_file, "{}  {}", hash, file_path.replace(
                            path, ""
                        )
                    )?;
                }
            }
    
            ChecksumAlerts::created(&path_file);
        }

        Ok(())
    }
    
    pub async fn compare_lines(contents: &str, path: &str, local_file: &str, no_checksum: bool) -> Result<(), Box<dyn Error>> {
        if !no_checksum {
            if let Some(url) = VarsBlock::get_checksum(contents).await {
                let mut line_has_errors = false;

                let local_hash_file = format!(
                    "{}{}", path, FileMisc::replace_extension(local_file, "sha256")
                ); 

                let (local_lines, local_total_lines) = Self::read_local_file(&local_hash_file)?;
                let (remote_lines, remote_total_lines) = Self::read_remote_file(&url).await?;

                UI::section_header("checksum validate");
    
                if local_total_lines != remote_total_lines {
                    line_has_errors = true;
                    ChecksumAlerts::line_count_is_different(local_total_lines, remote_total_lines);
                }
            
                for (_, (local, remote)) in local_lines.iter().zip(remote_lines.iter()).enumerate() {
                    if !local.contains(remote) {
                        line_has_errors = true;
                        ChecksumAlerts::is_different(local);
                    } else {
                        line_has_errors = false;
                        ChecksumAlerts::is_equal(local);
                    }
                }

                if !line_has_errors {
                    ChecksumAlerts::check_content(true);
                } else {
                    ChecksumAlerts::check_content(false);
                }
            }
        }
        
        Ok(())
    }

}
