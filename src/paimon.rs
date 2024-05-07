extern crate colored;

use colored::*;
use clap::Parser;
use std::error::Error;

use crate::{
    args_cli::Flags,
    utils::misc::Misc,
    configs::env::Env,
    ui::ui_base::PaimonUI,
    cmd::read_list::ReadList,
    render::render::RenderMarkdown,

    addons::{
        scrape::Scrape,
        ravenlib::Ravenlib, 
    },
};

pub struct Paimon;

impl Paimon {

    async fn get(
        run: &str,
        no_ignore: bool,
        no_comments: bool,
        no_open_link: bool,
        kindle: Option<String>
    ) -> Result<(), Box<dyn Error>> {
        if !run.starts_with("http") {
            ReadList::read_local_file(
                run, no_ignore, no_comments, no_open_link, kindle
            ).await?;
        } else {
            ReadList::read_remote_file(
                run, no_ignore, no_comments, no_open_link, kindle
            ).await?;
        }

        Ok(())
    }

    pub async fn init() -> Result<(), Box<dyn Error>> {
        if let Err(err) = Env::download_env_file(false).await {
            eprintln!("Error: {}", err.to_string().red());
        }

        let flags = Flags::parse();
        let url = flags.url.as_deref().unwrap_or_default();
        let run = flags.run.as_deref().unwrap_or_default();
        let options = flags.options.as_deref().unwrap_or_default();

        if !run.is_empty() {
            PaimonUI::header();
            
            if !Misc::check_is_user(run) {
                let _ = Paimon::get(
                    run, flags.no_ignore, flags.no_comments, flags.no_open_link, flags.kindle
                ).await;

                RenderMarkdown::render_and_save_file(run, flags.no_open_link);
            } else {
                let _ = Ravenlib::get(
                    run, flags.no_ignore, flags.no_comments
                ).await;
            }
        }

        let _ = Scrape::get(
            flags.scrape, url, flags.no_ignore, flags.no_comments
        ).await;

        let _ = Env::options_parser(options).await;

        Ok(())
    }

}
