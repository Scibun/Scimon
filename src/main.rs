mod cmd;
mod utils;
mod addons;
mod monlib;
mod configs;
mod args_cli;

use clap::Parser;
use std::error::Error;

use crate::args_cli::Flags;

use crate::utils::misc::Misc;

use crate::configs::env::Env;

use crate::addons::scrape::Scrape;

use crate::cmd::bootstrap::Paimon;

use crate::monlib::{
    api_get_list::ApiGetList,
    api_publish_list::ApiPublishList
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if let Err(err) = Env::download_env_file(false).await {
        eprintln!("Error: {}", err);
    }

    let args_parser: Flags = Flags::parse();

    let kindle_email = args_parser.kindle;
    let url = args_parser.url.as_deref().unwrap_or_default();
    let run = args_parser.run.as_deref().unwrap_or_default();

    if !run.is_empty() {
        Paimon::header();
        
        if !Misc::check_format(run) {
            let _ = Paimon::run(
                run, args_parser.noignore, args_parser.no_comments, kindle_email.to_owned()
            ).await;
        } else {
            ApiGetList::get(
                run, args_parser.noignore, args_parser.no_comments, kindle_email.to_owned()
            ).await?;
        }
    }

    let _ = Scrape::get(
        args_parser.scrape, url, args_parser.noignore, args_parser.no_comments, kindle_email,
    ).await;

    let _ = ApiPublishList::publish(
        args_parser.publish, args_parser.file, args_parser.title, args_parser.privacy
    );
    
    let options = &args_parser.options.as_deref().unwrap_or_default();
    Env::options_parser(options).await?;

    Ok(())
}
