extern crate colored;

use lettre_email::Email;
use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};
use mime::APPLICATION_PDF;
use std::path::Path;

use colored::*;

use crate::utils::file::FileUtils;

use crate::configs::env::Env;

use crate::utils::validation::Validate;

pub struct Kindle;

impl Kindle {

    pub fn send_kindle(kindle_email: &str, file: &str) -> Result<(), String> {
        if let Err(err) = Validate::validate_email(kindle_email) {
            println!("Error: {}", err);
        }
    
        if let Err(e) = FileUtils::check_file_exists(file) {
            println!("{}", e);
        }
    
        if FileUtils::is_file_over(file, 25) {
            println!("Error: The file is larger than 25 MB");
            return Ok(());
        }
    
        let file_path = Path::new(file);
        let file_name = FileUtils::get_file_name(file).unwrap_or_else(|err| {
            println!("{}", err);
            "".to_string()
        });
    
        let email = match Email::builder()
            .to(kindle_email)
            .from(Env::env_var("SMTP_USERNAME"))
            .subject("convert")
            .attachment_from_file(file_path, None, &APPLICATION_PDF)
            .and_then(|e| e.build()) {
                Ok(e) => e,
                Err(err) => {
                    return Err(
                        format!(
                            "Failed to build email: {:?}", err
                        )
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
                println!("Could not send Kindle: {:?}", e);
    
                Err(
                    format!("Could not send Kindle: {:?}", e)
                )
            }
        }
    }
    
}
