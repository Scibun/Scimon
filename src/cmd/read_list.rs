use reqwest;

use std::{
    error::Error,

    fs::{
        self,
        File,
    },

    io::{
        Read,
        Cursor,
        BufRead,
        BufReader,
    }
};

use crate::{
    cmd::download::Download,
    ui::errors_alerts::ErrorsAlerts,

    utils::{
        url::UrlMisc,
        validation::Validate
    },

    system::{
        syntax::Macros,
        providers::Providers,
    },
};

pub struct ReadList;

impl ReadList {

    async fn read_lines<R>(
        reader: R, 
        no_ignore: bool, 
        no_comments: bool, 
        no_open_link: bool
    ) -> Result<(), Box<dyn Error>> where R: BufRead {
        let mut path = String::new();

        for line_result in reader.lines() {
            let line = line_result?;
            let trimmed_line = line.trim();
    
            if !Macros::handle_check_macro_line(&trimmed_line, "open_link") {
                if path.is_empty() {
                    path = Macros::handle_get_path(trimmed_line);
                    let _ = fs::create_dir(&path);
                }
    
                let url = Providers::check_provider_line(&trimmed_line);
    
                Download::pdf(
                    &url,
                    &path,
                    no_ignore,
                    no_comments
                ).await?;
            } else {
                if !no_open_link {
                    UrlMisc::open_url(trimmed_line, true);
                }
            }
        }
    
        Ok(())
    }    
   
    pub async fn read_dataset(
        run: &str,
        no_ignore: bool,
        no_comments: bool,
        no_open_link: bool
    ) -> Result<(), Box<dyn Error>> {
        let reader: BufReader<Box<dyn Read>>;

        if run.starts_with("http") {
            let _ = Validate::validate_file_type(run, ".txt").map_err(|e| {
                ErrorsAlerts::generic(&e.to_string());
            });
            
            let response = reqwest::get(run).await?;
            let bytes = response.bytes().await?;
            let cursor = Cursor::new(bytes);

            reader = BufReader::new(
                Box::new(cursor)
            );
        } else {
            let _ = Validate::validate_file(run).map_err(|e| {
                ErrorsAlerts::generic(&e.to_string());
            });
            
            let file = File::open(run)?;

            reader = BufReader::new(
                Box::new(file)
            );
        }

        Self::read_lines(
            reader,
            no_ignore,
            no_comments,
            no_open_link
        ).await?;

        Ok(())
    }

}
