use std::{
    error::Error,

    io::{
        Read,
        Write,
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
    utils::file::FileMisc,
    ui::macros_alerts::MacrosAlerts,
};

pub struct Checksum;

impl Checksum {

    fn calculate_local_sha256(file_path: &str) -> Result<String, Box<dyn Error>> {
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

    pub async fn generate_hashes(path: &str, file: &str) -> Result<(), Box<dyn Error>> {
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

        MacrosAlerts::checksum(&path_file);
        Ok(())
    }

}
