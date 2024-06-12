use reqwest;
use std::{
    fs::File,
    error::Error,

    io::{
        Read, 
        Cursor,
    },
};

use crate::{
    args_cli::Flags, 
    utils::validation::Validate,
    ui::errors_alerts::ErrorsAlerts,

    monset::{
        runner_block::RunnerBlock,
        downloads_block::DownloadsBlock,
    }, 
};

pub struct Monset;

impl Monset {  

    pub async fn exec(run: &str, flags: &Flags) -> Result<(), Box<dyn Error>> {
        let mut buffer = Vec::new();

        if run.starts_with("http") {
            let _ = Validate::file_type(run, ".pbd").map_err(|e| {
                ErrorsAlerts::generic(&e.to_string());
            });
            
            let response = reqwest::get(run).await?;
            let bytes = response.bytes().await?;
            buffer.extend_from_slice(&bytes);
        } else {
            let _ = Validate::file(run).map_err(|e| {
                ErrorsAlerts::generic(&e.to_string());
            });

            let mut file = File::open(run)?;
            file.read_to_end(&mut buffer)?;
        }

        {
            let mut reader = Cursor::new(buffer.clone());
            DownloadsBlock::read_lines(&mut reader, flags, run).await?;
        }

        {
            let mut reader = Cursor::new(buffer);
            RunnerBlock::read_lines(&mut reader).await?;
        }

        Ok(())
    }

}
