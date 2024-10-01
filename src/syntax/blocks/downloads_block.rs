use is_url::is_url;

use std::{
    io::BufRead,
    error::Error,
    collections::HashSet,
};

use crate::{
    args_cli::Flags, 
    utils::file::FileUtils,
    system::providers::Providers,
    
    generator::{
        math::Math,
        covers::Covers,
        checksum::Checksum,
    },

    cmd::{
        tasks::Tasks,
        compress::Compress, 
    },

    syntax::{
        macros::Macros, vars::Vars,
        blocks::readme_block::ReadMeBlock, 
    }, 

    ui::{
        ui_base::UI,
        panic_alerts::PanicAlerts, 
        macros_alerts::MacrosAlerts, 
    }, 
};

pub struct DownloadsBlock;

impl DownloadsBlock {
    
    async fn block(contents: &str, downloads_content: &str, path: &str, flags: &Flags) -> Result<(), Box<dyn Error>> {
        let mut seen_urls = HashSet::new();

        for line in downloads_content.lines() {
            let url = line.trim().split_whitespace().next().unwrap_or("");
            let final_url = Providers::new(url).arxiv();

            if line.trim().starts_with("downloads {") {
                continue;
            } else if line.trim().starts_with("}") {
                break;
            }

            if seen_urls.contains(&final_url) {
                continue;
            }
        
            seen_urls.insert(final_url.to_string());

            if !Macros::handle_check_macro_line(&line, "ignore") {
                if !final_url.is_empty() && is_url(&final_url) && final_url.starts_with("http") {
                    Tasks::download(
                        Some(contents),
                        &final_url,
                        &path,
                        flags,
                    ).await?;
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

            Compress::new(&contents).get()?;
            Covers::new(&contents).get().await?;
            Tasks::qr_codes(&contents).await?;
            Math::new(&contents).render()?;
            
            Vars::get_open(&contents, flags.no_open_link).await;
            ReadMeBlock::render_var_and_save_file(&contents, flags).await?;

            Checksum::new(Some(contents)).files()?;
        } else {
            PanicAlerts::downloads_block();
        }

        Ok(())
    }

}
