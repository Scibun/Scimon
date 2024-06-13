use is_url::is_url;

use std::{
    io::BufRead,
    error::Error,
};

use crate::{
    args_cli::Flags,
    utils::file::FileUtils,
    system::providers::Providers, 
    ui::macros_alerts::MacrosAlerts,
    
    cmd::{
        tasks::Tasks,
        checksum::Checksum,
    },
    
    monset::{
        macros::Macros, 
        vars::Vars,
        blocks::readme_block::ReadMeBlock, 
    },
};

pub struct DownloadsBlock;

impl DownloadsBlock {

    pub async fn read_lines<R>(reader: R, flags: &Flags, checksum_file: &str) -> Result<(), Box<dyn Error>> where R: BufRead {
        let mut links = Vec::new();
        
        let contents = reader.lines().collect::<Result<Vec<_>, _>>()?.join("\n");
        let path = Vars::get_path(&contents);

        let start_index = match (contents.find("downloads {"), contents.find("downloads{")) {
            (Some(idx1), Some(idx2)) => Some(idx1.min(idx2)),
            (Some(idx), None) | (None, Some(idx)) => Some(idx),
            (None, None) => None,
        };

        let end_index = contents.rfind("}");

        if let (Some(start_index), Some(end_index)) = (start_index, end_index) {
            let mut refs = Vec::new();

            FileUtils::create_path(&path);
            let downloads_content = &contents[start_index + "downloads ".len()..end_index];

            if downloads_content.trim().starts_with("commands {") {
                return Ok(());
            }

            for line in downloads_content.lines() {
                let url = line.trim().split_whitespace().next().unwrap_or("");
                let final_url = Providers::check_provider_line(&url);

                if line.trim().starts_with("downloads {") {
                    continue;
                } else if line.trim().starts_with("}") {
                    break;
                }

                if !flags.no_comments && Macros::handle_check_macro_line(line, "debug") {
                    MacrosAlerts::comments(line);
                }

                if !Macros::handle_check_macro_line(&line, "ignore") {
                    if !final_url.is_empty() && is_url(&final_url) && final_url.starts_with("http") {
                        let success = Tasks::download(
                            &url,
                            &path,
                            flags,
                        ).await?;

                        refs.push(success);
                        let url_no_macros = Macros::remove_macros(&final_url);
                        links.push(url_no_macros.to_string());
                    }
                } else {
                    Macros::handle_ignore_macro_flag(&final_url, flags.no_ignore)?;
                }
            }

            Vars::get_open(&contents, flags.no_open_link).await;
            ReadMeBlock::render_var_and_save_file(&contents, flags).await?;

            Checksum::generate_hashes(&path, checksum_file, &refs, flags).await?;
            Checksum::compare_lines(&contents, checksum_file, flags).await?;

            Tasks::compress(&contents, &refs)?;
        } else {
            eprintln!("'downloads' block not found in file.");
        }

        Ok(())
    }

}
