extern crate chrono;

use reqwest;
use regex::Regex;

use std::{
    fs::File,
    io::Write,
    error::Error,

    process::{
        Stdio,
        Command,
    },
};

use crate::{
    system::system::System,
    regexp::regex_core::CoreRegExp,
    ui::errors_commands_alerts::ErrorsCommandsAlerts,

    utils::{
        file::FileUtils,
        remote::Remote,
    },
};

pub struct Scripts;

impl Scripts {

    pub async fn download(url: &str, path: &str) -> Result<String, Box<dyn Error>> {
        let response = reqwest::get(url).await?;
        
        if response.status().is_success() {
            let filename = Remote::get_filename(url, false).await?;
            
            let file_path = format!(
                "{}/{}", path, url.split("/").last().unwrap_or(&filename)
            );

            let mut file = File::create(&file_path)?;
            let content = response.bytes().await?;
    
            file.write_all(&content)?;
            return Ok(file_path)
        }

        Ok("".to_string())
    }
    
    pub fn exec(line: &str, program: &str) -> Result<(), Box<dyn Error>> {
        let line_cleanned = Regex::new(
            CoreRegExp::CLEAN_LINE
        ).unwrap().replace_all(
            &line, ""
        ).to_string();

        let output = Command::new(&program)
            .arg(line_cleanned)
            .stdout(Stdio::piped())
            .output()?;
        
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("{}", stdout);
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            ErrorsCommandsAlerts::executing(&stderr);
        }

        Ok(())
    }

    pub async fn read(line_trimmed: &str) -> Result<(), Box<dyn Error>> {
        if line_trimmed.len() >= 3 {
            let script = if line_trimmed.starts_with("http") {
                let path = System::SCRIPTS_FOLDER.to_str().unwrap_or_default().to_string();

                FileUtils::create_path(&path);
                Self::download(&line_trimmed, &path).await?
            } else {
                line_trimmed.to_string()
            };

            if script.ends_with(".py") {
                Self::exec(&script, "python")?;
            } else if line_trimmed.ends_with(".js") {
                Self::exec(&script, "node")?;
            } else {
                ErrorsCommandsAlerts::unsupported(&script);
            }
        }

        Ok(())
    }

}
