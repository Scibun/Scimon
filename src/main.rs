extern crate colored;

mod ui;
mod cmd;
mod utils;
mod monlib;
mod addons;
mod configs;
mod args_cli;

use colored::*;
use clap::Parser;
use std::error::Error;

use crate::{
    args_cli::Flags,
    configs::env::Env,
    cmd::paimon::Paimon,
    addons::scrape::Scrape,
    monlib::api_publish_list::ApiPublishList,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if let Err(err) = Env::download_env_file(false).await {
        let error = err.to_string().red();
        eprintln!("Error: {}", error);
    }

    let flags = Flags::parse();
    let kindle_email = flags.kindle;
    let url = flags.url.as_deref().unwrap_or_default();
    let run = flags.run.as_deref().unwrap_or_default();
    let options = flags.options.as_deref().unwrap_or_default();

    Paimon::core(run, flags.noignore, flags.no_comments, kindle_email.to_owned()).await;

    Scrape::get(flags.scrape, url, flags.noignore, flags.no_comments).await?;
    
    let _ = ApiPublishList::publish(flags.publish, flags.file, flags.title, flags.privacy);
    
    Env::options_parser(options).await?;

    Ok(())
}
