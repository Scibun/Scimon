pub mod pdf {

    use reqwest;
    use url::Url;
    use std::fs::File;
    use std::io::copy;
    use std::error::Error;

    pub async fn check_url_status(url: &str) -> Result<(), Box<dyn Error>> {
        let response = reqwest::get(url).await?;
    
        if response.status().as_u16() == 200 {
            Ok(())
        } else {
            Err(
                format!(
                    "Received a non-200 status: {}", 
                    response.status()
                ).into()
            )
        }
    }

    pub async fn download_and_detect_name(url: &str) -> Result<String, Box<dyn std::error::Error>> {
        check_url_status(url).await?; // Verifica o status antes de baixar

        let response = reqwest::get(url).await?;
        let content_disposition = response.headers().get("content-disposition");
        
        // Tente obter o nome do arquivo do cabeçalho 'content-disposition'
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
            let mut dest = File::create(&file_name)?;
            let content = response.bytes().await?;

            copy(&mut content.as_ref(), &mut dest)?;
            Ok(file_name)
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "It was not possible to detect the file name.",
            )))
        }
    }

}
