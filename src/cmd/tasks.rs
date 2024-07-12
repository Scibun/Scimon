use is_url::is_url;

use std::{
    fs::File,
    io::Read,
    borrow::Cow,
    error::Error,
};

use sha2::{
    Digest,
    Sha256,
};

use crate::{
    args_cli::Flags,
    monset::macros::Macros,

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

pub struct Tasks;

impl Tasks {

    pub fn hash_sha256(file_path: &str) -> Result<String, Box<dyn Error>> {
        let mut file = File::open(file_path)?;
        let mut hasher = Sha256::new();

        let mut buffer = [0; 1024];
        
        loop {
            let bytes_read = file.read(&mut buffer)?;

            if bytes_read == 0 { break; }

            hasher.update(&buffer[..bytes_read]);
        }
    
        let hash = hasher.finalize();
        Ok(format!("{:x}", hash))
    }

    pub async fn download(url: &str, path: &str, flags: &Flags) -> Result<String, Box<dyn Error>> {
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
                    let password = Pdf::is_pdf_encrypted(&file_path);
                    let hash = Self::hash_sha256(file_path)?;
                    
                    SuccessAlerts::download(&file, url, password, &hash);
                    return Ok(file_path.to_string())
                },

                Err(e) => ErrorsAlerts::download(e, url),
            }
        }

        Ok("".to_string())
    }

}
