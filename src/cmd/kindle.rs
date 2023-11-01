pub mod send{

    extern crate colored;

    use lettre_email::Email;
    use lettre::smtp::authentication::Credentials;
    use lettre::{SmtpClient, Transport};
    use mime::APPLICATION_PDF;
    use std::path::Path;

    use colored::*;

    use crate::cmd::env::configs::env_var;

    use crate::cmd::validation::data::validate_email;

    use crate::cmd::utils::utils_handler::{
        is_file_over,
        get_file_name,
        check_file_exists
    };

    pub fn send_kindle(kindle_email: &str, file: &str) -> Result<(), String> {
        if let Err(err) = validate_email(kindle_email) {
            println!("Error: {}", err);
        }

        if let Err(e) = check_file_exists(file) {
            println!("{}", e);
        }

        if is_file_over(file, 25) {
            println!("Error: The file is larger than 25 MB");
            return Ok(());
        }

        let file_path = Path::new(file);
        let file_name = get_file_name(file).unwrap_or_else(|err| {
            println!("{}", err);
            "".to_string()
        });

        let email = match Email::builder()
            .to(kindle_email)
            .from(env_var("SMTP_USERNAME"))
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
            env_var("SMTP_USERNAME").to_string(),
            env_var("SMTP_PASSWORD").to_string(),
        );

        let mut mailer = SmtpClient::new_simple(
            env_var("SMTP_SERVER").as_str()
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
