use reqwest;

use std::{
    fs::File,
    error::Error,

    io::{
        Read,
        Cursor,
        BufReader,
    },
};

use crate::{
    utils::validation::Validate,
    ui::errors_alerts::ErrorsAlerts,
    syntax::downloads_block::DownloadsBlock,
};

pub struct ReadList;

impl ReadList {    
   
    pub async fn read_dataset(
        run: &str,
        no_ignore: bool,
        no_comments: bool,
        no_open_link: bool,
    ) -> Result<(), Box<dyn Error>> {
        let reader: BufReader<Box<dyn Read>>;

        if run.starts_with("http") {
            let _ = Validate::file_type(run, ".txt").map_err(|e| {
                ErrorsAlerts::generic(&e.to_string());
            });
            
            let response = reqwest::get(run).await?;
            let bytes = response.bytes().await?;
            let cursor = Cursor::new(bytes);

            reader = BufReader::new(Box::new(cursor) as Box<dyn Read>);
        } else {
            let _ = Validate::file(run).map_err(|e| {
                ErrorsAlerts::generic(&e.to_string());
            });
            
            let file = File::open(run)?;

            reader = BufReader::new(Box::new(file) as Box<dyn Read>);
        }

        DownloadsBlock::read_lines(
            reader,
            no_ignore,
            no_comments,
            no_open_link
        ).await?;

        Ok(())
    }

}
