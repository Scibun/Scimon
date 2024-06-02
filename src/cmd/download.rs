use is_url::is_url;

use std::{
    borrow::Cow,
    error::Error,
};

use crate::{
    args_cli::Flags,
    syntax::macros::Macros,

    ui::{
        errors_alerts::ErrorsAlerts,
        success_alerts::SuccessAlerts,
    },

    system::{
        pdf::Pdf,
        hashes::Hashes,
        markdown::Markdown,
        reporting::Reporting,
        providers::Providers,
    },

};

pub struct Download;

impl Download {

    pub async fn file(url: &str, path: &str, flags: &Flags) -> Result<String, Box<dyn Error>> {
        let mut line_url = Cow::Borrowed(
            url.trim()
        );

        Reporting::check_download_errors(&line_url).await?;

        if !is_url(&line_url) { return Ok("".to_string()) }
    
        match Macros::handle_ignore_macro_flag(&line_url, flags.no_ignore) {
            Ok(new_line) => line_url = Cow::Owned(new_line),
            Err(_) => return Ok("".to_string()),
        }

        Markdown::create(&line_url, &path).await?;

        if Pdf::is_pdf_file(&line_url).await? || Providers::check_provider_domain(url) && !line_url.contains(".md") {
            let result = Pdf::download(&line_url, path).await;
            
            match result {
                Ok(file) => {
                    let file_path = &format!("{}{}", &path, &file);
                    let is_encrypted = Pdf::is_pdf_encrypted(&file_path);
                    let hash = Hashes::calculate_local_sha256(file_path)?;
                    
                    SuccessAlerts::download(&file, url, is_encrypted, &hash);
                    return Ok(file_path.to_string())
                },

                Err(e) => ErrorsAlerts::download(e, url),
            }
        }

        Ok("".to_string())
    }

}
