use is_url::is_url;

use std::{
    io::BufRead,
    error::Error,
};

use crate::{
    cmd::download::Download,
    syntax::vars_block::VarsBlock,

    system::{
        syntax::Macros,
        providers::Providers,
    },
};

pub struct DownloadsBlock;

impl DownloadsBlock {

    pub async fn read_lines<R>(
        reader: R, 
        no_ignore: bool, 
        no_comments: bool,
    ) -> Result<(), Box<dyn Error>> where R: BufRead {
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

                if !Macros::handle_check_macro_line(&line, "ignore") {
                    if !final_url.is_empty() && is_url(&final_url) && final_url.starts_with("http") {
                        Download::pdf(
                            &url,
                            &path,
                            no_ignore,
                            no_comments
                        ).await?;
                    }
                } else {
                    Macros::handle_ignore_macro_flag(&final_url, no_ignore)?;
                }
            }
        } else {
            eprintln!("'downloads' block not found in file.");
        }

        Ok(())
    }

}
