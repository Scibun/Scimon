use regex::Regex;

use std::{
    fs::File,
    io::Write,
    error::Error,
};

use crate::{
    args_cli::Flags,
    utils::file::FileMisc,
    system::hashes::Hashes,
    syntax::vars_block::VarsBlock,
    regexp::regex_core::CoreRegExp,

    ui::{
        ui_base::UI,
        checksum_alerts::ChecksumAlerts, 
    },
};

pub struct Checksum;

impl Checksum {
 
    pub fn extract_hashes_and_filenames(line: &str) -> Result<(String, String), Box<dyn Error>> {
        let re = Regex::new(CoreRegExp::GET_CHECKSUM).unwrap();
        let captures = re.captures(line).ok_or("No match found")?;

        let hash = captures.get(1).unwrap().as_str().to_string();
        let filename = captures.get(2).unwrap().as_str().to_string();

        Ok((hash, filename))
    }

    pub async fn generate_hashes(path: &str, file: &str, refs: Vec<String>, no_checksum: bool) -> Result<(), Box<dyn Error>> {
        if !no_checksum {
            let path_file = format!(
                "{}{}", path, FileMisc::replace_extension(file, "sha256")
            );
    
            let mut output_file = File::create(&path_file)?;

            for item in refs {
                let hash = Hashes::calculate_local_sha256(&item)?;

                writeln!(
                    output_file, "{}  {}", hash, item.replace(path, "")
                )?;
            }
    
            ChecksumAlerts::created(&path_file);
        }

        Ok(())
    }
    
    pub async fn compare_lines(contents: &str, path: &str, local_file: &str, flags: &Flags) -> Result<(), Box<dyn Error>> {
        if !flags.no_checksum && !flags.no_checksum_validate {
            if let Some(url) = VarsBlock::get_checksum(contents).await {
                let mut is_error = false;

                let local_hash_file = format!(
                    "{}{}", path, FileMisc::replace_extension(local_file, "sha256")
                ); 

                let (local_lines, local_total_lines) = Hashes::read_local_file(&local_hash_file)?;
                let (remote_lines, remote_total_lines) = Hashes::read_remote_file(&url).await?;

                UI::section_header("checksum validate");
    
                if local_total_lines != remote_total_lines {
                    is_error = true;
                    ChecksumAlerts::lines_total_is_different(local_total_lines, remote_total_lines);
                }
            
                for (_, (local, remote)) in local_lines.iter().zip(remote_lines.iter()).enumerate() {
                    if !local.contains(remote) {
                        is_error = true;
                        ChecksumAlerts::is_different(local);
                    } else {
                        is_error = false;
                        ChecksumAlerts::is_equal(local);
                    }
                }

                ChecksumAlerts::check_content(is_error);
            }
        }
        
        Ok(())
    }

}
