pub mod validation {

    use std::io;
    use reqwest;
    use std::env;
    use is_url::is_url;
    use std::path::Path;
    use std::error::Error;
    
    // Check if is a url 
    pub fn validate_url(url_line: &str) -> io::Result<()> {
        // Check if the line is a valid url
        if !is_url(url_line) {
            // Return a error
            return Err(
                io::Error::new(
                    io::ErrorKind::InvalidInput, format!(
                        "Error: The url {} is not a url valid", url_line
                    )
                )
            );
        }

        Ok(())
    }

    // Validate the paimon.txt file from used main.rs
    pub fn validate_file(file: &str) -> io::Result<()> {
        // Check if file exists
        if !Path::new(file).exists() {
            return Err(
                io::Error::new(
                    io::ErrorKind::NotFound, format!(
                        "Error: The file {} does not exist.", file
                    )
                )
            );
        }

        // Check if the file has a .txt extension.
        if !file.ends_with(".txt") {
            return Err(
                io::Error::new(
                    io::ErrorKind::InvalidData, format!(
                        "Error: The file {} is not a .txt", file
                    )
                )
            );
        }

        Ok(())
    }

    // Check tue url status from used in cmd::download::download::download_and_detect_name
    pub async fn check_url_status(url: &str) -> Result<(), Box<dyn Error>> {
        // Make a get request
        let response = reqwest::get(url).await?;
    
        // If status code was 200, continue runing
        if response.status().as_u16() == 200 {
            Ok(())
        } else {
            // Else, return a error
            Err(
                format!(
                    "Received a non-200 status: {}", 
                    response.status()
                ).into()
            )
        }
    }

    // Check if a arg was passed via cli from used in main.rs
    pub fn get_first_arg() -> Result<String, Box<dyn Error>> {
        // Get arguments passed via cli
        let args: Vec<String> = env::args().collect();
        
        if args.len() < 2 {
            Err(
                "No arguments provided".into()
            ) // If no exists a args, return a Error
        } else {
            Ok(
                args[1].clone()
            ) // Else, continue runing
        }
    }

}
