pub mod download {

    use reqwest;
    use url::Url;
    use std::fs::File;
    use std::io::copy;

    // Import the function's from cmd::validation:validation
    use crate::cmd::validation::validation::{
        check_url_status
    };

    // Function to download the file, used from main.rs
    pub async fn download_and_detect_name(url: &str) -> Result<String, Box<dyn std::error::Error>> {
        check_url_status(url).await?; // Check the status before downloading

        let response = reqwest::get(url).await?;

        let content_disposition = response.headers().get("content-disposition");
        
        // Try to get the file name from the 'content-disposition' header
        let filename = if let Some(value) = content_disposition {
            let cd_string = value.to_str()?;
            let parts: Vec<&str> = cd_string.split("filename=").collect();

            if parts.len() > 1 {
                Some(
                    parts[1].
                        trim_matches('"').
                        to_string()
                )
            } else {
                None
            }
        } else {
            let parsed_url = Url::parse(url)?;

            parsed_url.path_segments().and_then(
                |segments| segments.last()
            ).map(
                |name| name.to_string()
            )
        };

        if let Some(file_name) = filename {
            if file_name.ends_with(".pdf") {
                // Get destination file
                let mut dest = File::create(&file_name)?;

                // Get content file
                let content = response.bytes().await?;
    
                // Copty file
                copy(&mut content.as_ref(), &mut dest)?;
            } else {
                println!("Error: The file is not pdf type") // Return error message
            }

            Ok(file_name)
        } else {
            // Return error message
            Err(
                Box::new(
                    std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "Unable to detect the file name.",
                    )
                )
            )
        }
    }

}
