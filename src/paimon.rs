use clap::Parser;
use std::error::Error;

use crate::{
    args_cli::Flags,
    cmd::read_list::ReadList,
    prime_down::pd_render::PrimeDownRender,

    ui::{
        ui_base::UI,
        errors_alerts::ErrorsAlerts,
    },

    addons::{
        scrape::Scrape,
        ravenlib::Ravenlib, 
    },

    configs::{
        env::Env,
        settings::Settings,
        configs_files::ConfigsFiles,
    },
};

pub struct Paimon;

impl Paimon {
    
    async fn options(value: &str) -> Result<(), Box<dyn Error>> {
        if value == "open-env" {
            Env::open_env_file()?;
        } else if value == "open-settings" {
            Settings::open_settings_file()?;
        } else if value == "download-env" {
            ConfigsFiles::env_file(true, true).await?;
        } else if value == "download-settings" {
            ConfigsFiles::settings_file(true, true).await?;
        }
        
        Ok(())
    }

    pub async fn init() {
        if let Err(err) = ConfigsFiles::env_file(false, false).await {
            ErrorsAlerts::generic(&err.to_string());
        }

        if let Err(err) = ConfigsFiles::settings_file(false, false).await {
            ErrorsAlerts::generic(&err.to_string());
        }

        let flags = Flags::parse();
        let url = flags.url.as_deref().unwrap_or_default();
        let run = flags.run.as_deref().unwrap_or_default();
        let options = flags.options.as_deref().unwrap_or_default();

        UI::header();
        
        if !run.is_empty() {
            if !Ravenlib::check_is_user(run) {
                let _ = ReadList::read_dataset(
                    run, flags.no_ignore, flags.no_comments, flags.no_open_link
                ).await;

                PrimeDownRender::render_and_save_file(run, flags.no_open_link);
            } else {
                let _ = Ravenlib::get(
                    run, flags.no_ignore, flags.no_comments
                ).await;
            }
        }

        let _ = Scrape::get(
            flags.scrape, url, flags.no_ignore, flags.no_comments
        ).await;

        let _ = Self::options(options).await;
    }

}
