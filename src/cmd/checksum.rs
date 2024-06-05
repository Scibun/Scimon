use std::{
    io::Write,
    error::Error,

    fs::{
        self,
        File,
    },
};

use crate::{
    args_cli::Flags,
    utils::file::FileMisc,
    system::hashes::Hashes,
    syntax::vars_block::VarsBlock,

    ui::{
        ui_base::UI,
        checksum_alerts::ChecksumAlerts, 
    },
};

pub struct Checksum;

impl Checksum {

    fn checksum_unmatch_delete_file(contents: &str, path: &str, line: &str) {
        if let Some(value) = VarsBlock::get_checksum_unmatch(contents) {
            if value == "delete" {
                let file = Hashes::extract_filename(line).unwrap();
                fs::remove_file(format!("{}{}", path, file)).unwrap();
                ChecksumAlerts::lines_unmatch_file_deleted(&file);
            }
        }
    }

    pub async fn generate_hashes(path: &str, file: &str, refs: Vec<String>, flags: &Flags) -> Result<(), Box<dyn Error>> {
        if !flags.no_checksum {
            let path_file = format!(
                "{}{}", path, FileMisc::replace_extension(
                    file, "sha256"
                )
            );
    
            let mut output_file = File::create(&path_file)?;

            for file_path in refs {
                let hash = Hashes::calculate_local_sha256(&file_path)?;

                writeln!(
                    output_file, "{}  {}", hash, file_path.replace(
                        path, ""
                    )
                )?;
            }
    
            ChecksumAlerts::created(&path_file);
        }

        Ok(())
    }
    
    pub async fn compare_lines(contents: &str, checksum_file: &str, flags: &Flags) -> Result<(), Box<dyn Error>> {
        if !flags.no_checksum && !flags.no_checksum_validate {
            let path = VarsBlock::get_path(contents);

            if let Some(url) = VarsBlock::get_checksum(contents).await {
                let local_hash_file = format!(
                    "{}{}", path, FileMisc::replace_extension(checksum_file, "sha256")
                ); 

                let (local_lines, local_total_lines) = Hashes::read_local_file(&local_hash_file)?;
                let (remote_lines, remote_total_lines) = Hashes::read_remote_file(&url).await?;

                UI::section_header("checksum validate");
                ChecksumAlerts::lines_total_is_different(local_total_lines, remote_total_lines);
            
                for (_, (local, remote)) in local_lines.iter().zip(remote_lines.iter()).enumerate() {
                    if !local.contains(remote) {
                        ChecksumAlerts::is_different(local);
                        Self::checksum_unmatch_delete_file(contents, &path, local);
                    } else {
                        ChecksumAlerts::is_equal(local);
                    }
                }
            }
        }
        
        Ok(())
    }

}
