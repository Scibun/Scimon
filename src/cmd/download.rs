pub mod file {

    use reqwest;
    use url::Url;
    use std::fs::File;
    use std::io::copy;

    use crate::cmd::validation::data::{
        check_url_status,
        validate_file_type
    };

    pub async fn download_and_detect_name(url: &str) -> Result<String, Box<dyn std::error::Error>> {
        check_url_status(url).await?;

        let response = reqwest::get(url).await?;

        let content_disposition = response.headers().get("content-disposition");
        
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
            let _ = validate_file_type(&file_name, ".pdf");

            let mut dest = File::create(&file_name)?;
            let content = response.bytes().await?;
            copy(&mut content.as_ref(), &mut dest)?;

            Ok(file_name)
        } else {
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
