use walkdir::WalkDir;

use std::{
    fs::File,
    path::Path,
    error::Error,

    io::{
        Read,
        Result as IoResult,
    },
};

use sha2::{
    Digest,
    Sha256,
};

use crate::{
    syntax::vars::Vars, 
    
    ui::{
        ui_base::UI,
        success_alerts::SuccessAlerts, 
    }
};

pub struct Checksum;

impl Checksum {

    pub fn hash_sha256(file_path: &str) -> Result<String, Box<dyn Error>> {
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
    
    pub fn files(contents: &str) -> IoResult<()> {
        UI::section_header("Hashes files", "normal");
        let folder_path = Vars::get_path(contents);

        for entry in WalkDir::new(&folder_path) {
            let entry = entry?;
            let path = entry.path();
            let name = path.strip_prefix(Path::new(&folder_path)).unwrap();

            if path.extension().map_or(false, |ext| ext == "pdf") {
                let file = name.to_str().unwrap();
                let path = path.to_str().unwrap();
                let hash = Checksum::hash_sha256(path).unwrap();

                SuccessAlerts::hash(&file, &hash);
            }
        }

        Ok(())
    }

}
