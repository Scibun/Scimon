use is_url::is_url;

use std::{
    borrow::Cow,
    error::Error,
};

use crate::{
    syntax::macros::Macros,

    ui::{
        errors_alerts::ErrorsAlerts,
        success_alerts::SuccessAlerts,
    },

    system::{
        pdf::Pdf,
        markdown::Markdown,
        reporting::Reporting,
        providers::Providers,
    },

};

pub struct Download;

impl Download {

    pub async fn pdf(url: &str, path: &str, no_ignore: bool) -> Result<(), Box<dyn Error>> {
        let mut line_url: Cow<str> = Cow::Borrowed(
            url.trim()
        );

        Reporting::check_download_errors(&line_url).await?;

        if !is_url(&line_url) {
            return Ok(())
        }
    
        match Macros::handle_ignore_macro_flag(&line_url, no_ignore) {
            Ok(new_line) => line_url = Cow::Owned(new_line),
            Err(_) => return Ok(()),
        }

        Markdown::create(&line_url, &path).await?;

        if Pdf::is_pdf_file(&line_url).await? || Providers::check_provider_domain(url) && !line_url.contains(".md") {
            let result = Pdf::download(&line_url, path).await;
            
            match result {
                Ok(file) => {
                    let is_encrypted = Pdf::is_pdf_encrypted(&format!("downloads/{}", &file));
                    SuccessAlerts::download(&file, url, is_encrypted)
                },
                Err(e) => ErrorsAlerts::download(e, url),
            }
        }

        Ok(())
    }

}
