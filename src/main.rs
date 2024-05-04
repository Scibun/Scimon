mod api;
mod cmd;
mod utils;
mod configs;
mod args_cli;

use clap::Parser;
use std::error::Error;

use crate::args_cli::Flags;

use crate::utils::misc::Misc;

use crate::configs::env::Env;

use crate::cmd::bootstrap::Paimon;

use crate::api::{
    api_get_list::ApiGetList, 
    api_publish_list::ApiPublishList
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if let Err(err) = Env::download_env_file(false).await {
        eprintln!("Error: {}", err);
    }

    let args_parser: Flags = Flags::parse();
    let run = args_parser.run.as_deref().unwrap_or_default();

    if !run.is_empty() {
        Paimon::header();
        
        if !Misc::check_format(run) {
            let _ = Paimon::read_paimon_file(
                run, args_parser.noignore, args_parser.no_comments, args_parser.kindle
            ).await;
        } else {
            ApiGetList::get(
                run, args_parser.noignore, args_parser.no_comments, args_parser.kindle
            ).await?;
        }
    }

    if args_parser.publish {
        if let (Some(file_path), Some(title)) = (&args_parser.file, &args_parser.title) {
            let privacy = args_parser.privacy.clone();

            let _ = ApiPublishList::publish(
                file_path, title, privacy.as_deref()
            ).await;
        } else {
            eprintln!("Error: Both 'file' and 'title' are required for publishing a library.");
        }
    }
    
    let options = &args_parser.options.as_deref().unwrap_or_default();
    Env::options_parser(options).await?;

    Ok(())
}
