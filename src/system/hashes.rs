use std::{
    fs::File,
    io::Read,
    error::Error,
};

use sha2::{
    Digest,
    Sha256,
};

pub struct Hashes;

impl Hashes {

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
