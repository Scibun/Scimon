extern crate colored;

use colored::*;

use lettre_email::Email;

use lettre::{
    Transport,
    SmtpClient, 
    smtp::authentication::Credentials
};

use std::path::Path;
use mime::APPLICATION_PDF;

use crate::{
    configs::env::Env,

    utils::{
        url::UrlMisc,
        file::FileMisc,
        validation::Validate
    }
};

pub struct Kindle;

impl Kindle {

    fn execute(kindle_email: &str, file: &str) -> Result<(), String> {
        if let Err(e) = Validate::validate_email(kindle_email) {
            println!("Error: {}", e.red());
        }
    
        if let Err(e) = FileMisc::check_file_exists(file) {
            println!("Error: {}", e.red());
        }
    
        if FileMisc::is_file_over(file, 25) {
            println!("Error: {}", "The file is larger than 25 MB".red());
            return Ok(());
        }
    
        let file_path = Path::new(file);
        let file_name = FileMisc::get_file_name(file).unwrap_or_else(|e| {
            println!("{}", e.red());
            "".to_string()
        });
    
        let email = match Email::builder()
            .to(kindle_email)
            .from(Env::env_var("SMTP_USERNAME"))
            .subject("convert")
            .attachment_from_file(file_path, None, &APPLICATION_PDF)
            .and_then(|e| e.build()) {
                Ok(e) => e,
                Err(e) => {
                    return Err(
                        format!("Failed to build email: {:?}", e.to_string().red())
                    );
                }
            };
    
        let creds = Credentials::new(
            Env::env_var("SMTP_USERNAME").to_string(),
            Env::env_var("SMTP_PASSWORD").to_string(),
        );
    
        let mut mailer = SmtpClient::new_simple(
            Env::env_var("SMTP_SERVER").as_str()
        )
            .unwrap()
            .credentials(creds)
            .transport();
    
        match mailer.send(email.into()) {
            Ok(_) => {
                println!("-> Document sent to the Kindle, file: {}", file_name.green());
                Ok(())
            },
            
            Err(e) => {
                println!("Could not send Kindle: {:?}", e.to_string().red());
    
                Err(
                    format!("Could not send Kindle: {:?}", e)
                )
            }
        }
    }
    
    pub fn send(url: &str, path: &str, kindle_email: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(filename) = UrlMisc::extract_file_name(&url) {
            let kindle_filename = FileMisc::get_output_path(&path, &filename);

            if let Some(kindle_filename_str) = kindle_filename.to_str() {
                self::Kindle::execute(&kindle_email, kindle_filename_str)?; // Prefixo self::
                Ok(())
            } else {
                Err("Failed to convert output path to string".into())
            }
        } else {
            Err("Failed to extract file name from URL".into())
        }
    }

}
