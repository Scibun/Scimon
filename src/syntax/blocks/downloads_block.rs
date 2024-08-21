use is_url::is_url;

use std::{
    io::BufRead,
    error::Error,
};

use crate::{
    args_cli::Flags,
    utils::file::FileUtils,
    system::providers::Providers, 
    ext::extract_covers::ExtractCovers,

    cmd::{
        tasks::Tasks,
        compress::Compress,
    },

    ui::{
        ui_base::UI,
        panic_alerts::PanicAlerts,
        macros_alerts::MacrosAlerts,
    },
    
    syntax::{
        vars::Vars,
        macros::Macros, 
        blocks::readme_block::ReadMeBlock, 
    },
};

pub struct DownloadsBlock;

impl DownloadsBlock {
    
    async fn block(contents: &str, downloads_content: &str, path: &str, flags: &Flags) -> Result<(), Box<dyn Error>> {
        for line in downloads_content.lines() {
            let url = line.trim().split_whitespace().next().unwrap_or("");
            let final_url = Providers::arxiv(&url);

            if line.trim().starts_with("downloads {") {
                continue;
            } else if line.trim().starts_with("}") {
                break;
            }

            if !Macros::handle_check_macro_line(&line, "ignore") {
                if !final_url.is_empty() && is_url(&final_url) && final_url.starts_with("http") {
                    Tasks::download(
                        Some(contents),
                        &url,
                        &path,
                        flags,
                    ).await?;

                    Tasks::qr_code(contents, final_url.clone())?;
                }
            } else {
                MacrosAlerts::ignore(&final_url);
            }
        }

        Ok(())
    }

    pub async fn read_lines<R>(reader: R, flags: &Flags) -> Result<(), Box<dyn Error>> where R: BufRead {
        let contents = reader.lines().collect::<Result<Vec<_>, _>>()?.join("\n");
        let path = Vars::get_path(&contents);

        let start_index = match (contents.find("downloads {"), contents.find("downloads{")) {
            (Some(idx1), Some(idx2)) => Some(idx1.min(idx2)),
            (Some(idx), None) | (None, Some(idx)) => Some(idx),
            (None, None) => None,
        };

        let end_index = contents.rfind("}");

        if let (Some(start_index), Some(end_index)) = (start_index, end_index) {
            FileUtils::create_path(&path);
            let downloads_content = &contents[start_index + "downloads ".len()..end_index];

            if downloads_content.trim().starts_with("commands {") {
                return Ok(());
            }

            UI::section_header("downloads", "normal");
            Self::block(&contents, downloads_content, &path, flags).await?;

            Compress::files(&contents)?;
            ExtractCovers::extract(&contents).await?;
            Vars::get_open(&contents, flags.no_open_link).await;
            ReadMeBlock::render_var_and_save_file(&contents, flags).await?;
        } else {
            PanicAlerts::downloads_block();
        }

        Ok(())
    }

}
