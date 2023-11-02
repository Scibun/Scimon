use is_url::is_url;

use std::fs::File;
use std::error::Error;
use std::io::{BufReader, BufRead};

use crate::utils::validation::{
    validate_file,
    validate_file_type
};

use crate::cmd::download::run_download_current_line;

pub async fn read_paimon_local_file(file_path: &str, no_ignore: bool, kindle: Option<String>) -> Result<(), Box<dyn Error>> {
    if let Err(e) = validate_file(file_path) {
        eprintln!("{}", e);
        return Err(Box::new(e));
    }

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result?;
        let _ = run_download_current_line(&line, no_ignore, kindle.clone()).await?;
    }

    Ok(())
}

pub async fn read_paimon_remote_file(url: &str, no_ignore: bool, kindle: Option<String>) -> Result<(), Box<dyn Error>> {
    validate_file_type(url, ".txt")?;

    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;

    let reader = BufReader::new(bytes.as_ref());

    for line_result in reader.lines() {
        let line = line_result?;
        let _ = run_download_current_line(&line, no_ignore, kindle.clone()).await?;
    }

    Ok(())
}

pub async fn read_paimon_file(run: &str, no_ignore: bool, kindle: Option<String>) {
    if !is_url(run) {
        if let Err(_) = read_paimon_local_file(
            run, no_ignore, kindle
        ).await {}
    } else {
        if let Err(e) = read_paimon_remote_file(
            &run, no_ignore, kindle
        ).await {
            eprintln!("Error: {}", e);
        }
    }
}
