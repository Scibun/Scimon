use is_url::is_url;

use std::{
    io::BufRead,
    error::Error,
};

use crate::{
    system::providers::Providers,
    ui::macros_alerts::MacrosAlerts,

    cmd::{
        checksum::Checksum,
        download::Download,
    },

    syntax::{
        macros::Macros,
        vars_block::VarsBlock,
        readme_block::ReadMeBlock,
    },
};

pub struct DownloadsBlock;

impl DownloadsBlock {

    pub async fn read_lines<R>(
        reader: R, 
        no_ignore: bool, 
        no_comments: bool,
        no_open_link: bool,
        no_checksum: bool,
        no_readme: bool,
        checksum_file: &str,
    ) -> Result<(), Box<dyn Error>> where R: BufRead {
        let mut links = Vec::new();
        
        let contents = reader.lines().collect::<Result<Vec<_>, _>>()?.join("\n");
        let path = VarsBlock::get_path(&contents);

        let start_index = match (contents.find("downloads {"), contents.find("downloads{")) {
            (Some(idx1), Some(idx2)) => Some(idx1.min(idx2)),
            (Some(idx), None) | (None, Some(idx)) => Some(idx),
            (None, None) => None,
        };

        let end_index = contents.rfind("}");

        if let (Some(start_index), Some(end_index)) = (start_index, end_index) {
            let downloads_content = &contents[start_index + "downloads ".len()..end_index];

            for line in downloads_content.lines() {
                let url = line.trim().split_whitespace().next().unwrap_or("");
                let final_url = Providers::check_provider_line(&url);

                if line.trim().starts_with("downloads {") {
                    continue;
                } else if line.trim().starts_with("}") {
                    break;
                }

                if !no_comments && line.contains("!debug") {
                    MacrosAlerts::comments(line);
                }

                if !Macros::handle_check_macro_line(&line, "ignore") {
                    if !final_url.is_empty() && is_url(&final_url) && final_url.starts_with("http") {
                        Download::file(
                            &url,
                            &path,

                            no_ignore,
                        ).await?;

                        let url_no_macros = Macros::remove_macros(&final_url);
                        links.push(url_no_macros.to_string());
                    }
                } else {
                    Macros::handle_ignore_macro_flag(&final_url, no_ignore)?;
                }
            }

            if !no_open_link {
                VarsBlock::get_open(&contents).await;
            }

            if !no_readme {
                ReadMeBlock::render_var_and_save_file(&contents, no_open_link).await?;
            }

            if !no_checksum {
                Checksum::generate_hashes(&path, checksum_file).await?;
            }
        } else {
            eprintln!("'downloads' block not found in file.");
        }

        Ok(())
    }

}
