use is_url::is_url;

use std::{
    fs::File,
    borrow::Cow,
    error::Error,

    io::{
        Read,
        BufRead,
    },
};

use sha2::{
    Digest,
    Sha256,
};

use crate::{
    args_cli::Flags,
    configs::settings::Settings,
    generator::qr_code::GenQrCode,
    ui::success_alerts::SuccessAlerts,

    utils::{
        file::FileUtils,
        file_name_remote::FileNameRemote,
    },
    
    syntax::{
        vars::Vars,
        macros::Macros,
    },

    system::{
        pdf::Pdf,
        markdown::Markdown,
        reporting::Reporting,
    },
};

pub struct Tasks;

impl Tasks {

    pub async fn prints<R>(reader: R) -> Result<(), Box<dyn Error>> where R: BufRead, {
        let contents = reader.lines().collect::<Result<Vec<_>, _>>()?.join("\n");

        for line in contents.lines() {
            Vars::get_print(&line);
        }

        Ok(())
    }

    pub fn qr_code(contents: &str, url: String) -> Result<(), Box<dyn Error>> {
        if let Some(qrcode_path) = Vars::get_qrcode(contents) {
            FileUtils::create_path(&qrcode_path);

            let value = Settings::get("general.qrcode_size", "INT");
            let qrcode_size = value.as_i64().expect("Invalid qrcode_size value. Must be an integer.") as usize;

            let name = FileNameRemote::new(url.as_str()).get();
            let name_pdf = FileUtils::replace_extension(&name, "png");
            let file_path = format!("{}{}", qrcode_path, name_pdf);
            
            GenQrCode::new(&url, qrcode_size).png(&file_path).unwrap();
            SuccessAlerts::qrcode(file_path.as_str());
        }
        
        Ok(())
    }

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

    pub async fn download(contents: Option<&str>, url: &str, path: &str, flags: &Flags) -> Result<String, Box<dyn Error>> {
        let mut line_url = Cow::Borrowed(
            url.trim()
        );

        Reporting::check_download_errors(&line_url).await?;

        if !is_url(&line_url) { return Ok("".to_string()) }
    
        match Macros::handle_ignore_macro_flag(&line_url, flags.no_ignore) {
            Ok(new_line) => line_url = Cow::Owned(new_line),
            Err(_) => return Ok("".to_string()),
        }

        if let Some(contents) = contents {
            Markdown::create(&contents, &line_url, &path).await?;
        }

        Pdf::download_line(&line_url, url, path).await?;

        Ok("".to_string())
    }

}
