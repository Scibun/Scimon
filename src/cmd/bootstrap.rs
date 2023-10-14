pub mod file_handler {

    use std::fs::File;
    use std::error::Error;
    use std::io::{BufReader, BufRead};

    use crate::cmd::validation::data::{
        validate_url, 
        validate_file,
        validate_file_type
    };

    use crate::cmd::download::file::download_and_detect_name;

    async fn run_download_current_line(line: &str) -> Result<(), Box<dyn Error>> {
        if line.trim().is_empty() { return Ok(()); }

        if line.to_lowercase().contains("!comment") || line.starts_with("//") {
            if line.to_lowercase().contains("!show_comment".trim()) {
                eprintln!("{}", &line.replace("!show_comment".trim(), ""))
            }

            return Ok(())
        }

        if line.to_lowercase().contains("!ignore") {
            eprintln!(
                "🚫 The file {} was ignored", &line.trim().replace(
                    " !ignore", ""
                )
            );
            return Ok(())
        }

        if let Err(e) = validate_url(&line) {
            eprintln!("{}", e);
            return Err(Box::new(e))
        }

        let result = download_and_detect_name(&line).await;
        
        match result {
            Ok(file_name) => {
                println!("Downloaded file name: {}", file_name);
                return Ok(())
            },
            
            Err(e) => {
                eprintln!("Error downloading or detecting the name: {}", e);
                return Err(e);
            }
        }
    }

    pub async fn read_local_file(file_path: &str) -> Result<(), Box<dyn Error>> {
        if let Err(e) = validate_file(file_path) {
            eprintln!("{}", e);
            return Err(Box::new(e));
        }

        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        for line_result in reader.lines() {
            let line = line_result?;
            let _ = run_download_current_line(&line).await?;
        }

        Ok(())
    }

    pub async fn read_remote_file(url: &str) -> Result<(), Box<dyn Error>> {
        validate_file_type(url, ".txt")?;

        let response = reqwest::get(url).await?;
        let bytes = response.bytes().await?;

        let reader = BufReader::new(bytes.as_ref());

        for line_result in reader.lines() {
            let line = line_result?;
            let _ = run_download_current_line(&line).await?;
        }

        Ok(())
    }

}
