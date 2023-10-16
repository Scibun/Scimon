pub mod file {

    use reqwest;
    use url::Url;
    use std::fs::File;
    use std::io::copy;
    use is_url::is_url;
    use std::borrow::Cow;
    use std::error::Error;

    use crate::cmd::validation::data::{
        validate_url,
        check_url_status,
        validate_file_type
    };

    use crate::cmd::syntax::macros_flags::{
        handle_comments,
        handle_ignore_macro_flag
    };

    async fn download_and_detect_name(url: &str) -> Result<String, Box<dyn std::error::Error>> {
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

    pub async fn run_download_current_line(line: &str, params: &str) -> Result<(), Box<dyn Error>> {
        let mut processed_line: Cow<str> = Cow::Borrowed(
            line.trim()
        );

        let _ = handle_comments(&processed_line);
        if !is_url(&processed_line) { return Ok(()); }

        let result_ignore_macro_flag = handle_ignore_macro_flag(&processed_line, params);
        match result_ignore_macro_flag {
            Ok(new_line) => processed_line = Cow::Owned(new_line),
            Err(_) => return Ok(()),
        }

        if let Err(e) = validate_url(&processed_line) {
            eprintln!("{}", e);

            return Err(
                Box::new(e)
            )
        }

        let result = download_and_detect_name(&processed_line).await;

        match result {
            Ok(file_name) => {
                println!("-> Downloaded file name: {}", file_name);
                return Ok(())
            },

            Err(e) => {
                eprintln!("-> Error downloading or detecting the name: {}", e);
                return Err(e);
            }
        }
    }

}
