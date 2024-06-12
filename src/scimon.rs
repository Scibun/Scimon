use clap::Parser;
use std::error::Error;

use crate::{
    args_cli::Flags,
    cmd::monset::Monset,
    monset::blocks::readme_block::ReadMeBlock,

    ui::{
        ui_base::UI,
        errors_alerts::ErrorsAlerts,
    },

    addons::{
        scrape::Scrape,
        scibun::Scibun, 
    },

    configs::{
        env::Env,
        settings::Settings,
        configs_files::DownloadConfigsFiles,
    },
};

pub struct Scimon;

impl Scimon {
    
    async fn options(options: &str) -> Result<(), Box<dyn Error>> {
        if options == "open-env" {
            Env::open_env_file()?;
        } else if options == "open-settings" {
            Settings::open_settings_file()?;
        } else if options == "download-env" {
            DownloadConfigsFiles::env_file(true, true).await?;
        } else if options == "download-settings" {
            DownloadConfigsFiles::settings_file(true, true).await?;
        }
        
        Ok(())
    }

    pub async fn init() {
        let (print, force_mode) = (false, false);

        if let Err(err) = DownloadConfigsFiles::env_file(print, force_mode).await {
            ErrorsAlerts::generic(&err.to_string());
        }

        if let Err(err) = DownloadConfigsFiles::settings_file(print, force_mode).await {
            ErrorsAlerts::generic(&err.to_string());
        }

        let flags = Flags::parse();
        let url = flags.url.as_deref().unwrap_or_default();
        let run = flags.run.as_deref().unwrap_or_default();
        let options = flags.options.as_deref().unwrap_or_default();

        UI::header();
        
        if !run.is_empty() {
            if !Scibun::check_is_user(run) {
                let _ = Monset::exec(run, &flags).await;
                let _ = ReadMeBlock::render_block_and_save_file(run, &flags);
            } else {
                let _ = Scibun::get(run, &flags).await;
            }
        }

        let _ = Scrape::get(&flags, url).await;
        let _ = Self::options(options).await;
    }

}
