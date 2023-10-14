pub mod file_handler {

    use std::fs::File;
    use std::io::{self, BufReader, BufRead, Error};

    use crate::cmd::validation::data::{
        validate_url, 
        validate_file,
        validate_file_type
    };

    use crate::cmd::download::file::download_and_detect_name;

    pub async fn read_local_file(file_path: &str) -> Result<(), Error> {
        if let Err(e) = validate_file(file_path) {
            eprintln!("{}", e);
            return Ok(());
        }

        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        for line_result in reader.lines() {
            let line = match line_result {
                Ok(l) => l,

                Err(e) => {
                    eprintln!("Error: reading line: {}", e);
                    continue;
                }
            };

            if let Err(e) = validate_url(&line) {
                eprintln!("{}", e);
                return Ok(());
            }

            let result = download_and_detect_name(&line).await;
            
            match result {
                Ok(file_name) => {
                    println!("Downloaded file name: {}", file_name);
                },

                Err(e) => {
                    eprintln!("Error downloading or detecting the name: {}", e);
                }
            }
        }

        Ok(())
    }

    pub async fn read_remote_file(url: &str) -> io::Result<()> {
        let _ = validate_file_type(url, ".txt");

        let response = reqwest::get(url).await.map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("Failed to fetch the file: {}", e))
        })?;

        let bytes = response.bytes().await.map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("Failed to read the response content: {}", e))
        })?;

        let reader = BufReader::new(bytes.as_ref());
    
        for line in reader.lines() {
            match line {
                Ok(content) => {
                    if let Err(e) = validate_url(&content) {
                        eprintln!("{}", e);
                        return Ok(());
                    }
            
                    let result = download_and_detect_name(&content).await;
                    
                    match result {
                        Ok(file_name) => {
                            println!("Downloaded file name: {}", file_name);
                        },
                        Err(e) => {
                            eprintln!("Error downloading or detecting the name: {}", e);
                        }
                    }
                }
                
                Err(e) => {
                    eprintln!("Failed to read a line: {}", e);
                }
            }
        }
    
        Ok(())
    }

}
