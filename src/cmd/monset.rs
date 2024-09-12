use reqwest;
use is_url::is_url;

use std::{
    fs::File,
    error::Error,

    io::{
        Read, 
        Cursor
    },
};

use crate::{
    args_cli::Flags,
    cmd::tasks::Tasks,
    utils::validation::Validate,
    ui::errors_alerts::ErrorsAlerts,

    syntax::blocks::{
        runner_block::RunnerBlock, 
        downloads_block::DownloadsBlock
    },
};

pub struct Monset;

impl Monset {

    async fn read_file(run: &str) -> Result<Cursor<Vec<u8>>, Box<dyn Error>> {
        let mut buffer = Vec::new();

        if is_url(run) {
            let response = reqwest::get(run).await?;

            if !response.status().is_success() {
                ErrorsAlerts::generic(
                    &format!("Error while retrieving remote file: {}", response.status())
                );
            }

            let bytes = response.bytes().await?;
            buffer.extend_from_slice(&bytes);
        } else {
            let _ = Validate::file(run).map_err(|e| {
                ErrorsAlerts::generic(&e.to_string());
            });

            let mut file = File::open(run)?;
            file.read_to_end(&mut buffer)?;
        }

        Ok(Cursor::new(buffer))
    }

    pub async fn prints(run: &str) -> Result<(), Box<dyn Error>> {
        let reader = Self::read_file(run).await?;
        let _ = Tasks::prints(reader).await?;

        Ok(())
    }

    pub async fn downloads(run: &str, flags: &Flags) -> Result<(), Box<dyn Error>> {
        let mut reader = Self::read_file(run).await?;
        let _ = DownloadsBlock::read_lines(&mut reader, flags).await?;
        let _ = Tasks::prints(reader).await?;

        Ok(())
    }

    pub async fn run_code(run: &str) -> Result<(), Box<dyn Error>> {
        let mut reader = Self::read_file(run).await?;
        RunnerBlock::read_lines(&mut reader).await?;

        Ok(())
    }
}
