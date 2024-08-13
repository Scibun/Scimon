extern crate chrono;

use regex::Regex;

use std::{
    error::Error,

    process::{
        Stdio,
        Command,
    },
};

use crate::{
    consts::folders::Folders,
    system::plataforms::Plataforms,
    regexp::regex_core::CoreRegExp,
    ui::errors_commands_alerts::ErrorsCommandsAlerts,

    utils::{
        remote::Remote,
        file::FileUtils,
    },
};

pub struct Scripts;

impl Scripts {
    
    fn exec(line: &str, program: &str) -> Result<(), Box<dyn Error>> {
        let language = Plataforms::get_bin_name(program);

        let line_cleanned = Regex::new(
            CoreRegExp::CLEAN_LINE
        ).unwrap().replace_all(
            &line, ""
        ).to_string();

        let output = Command::new(&language)
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
                let path = Folders::SCRIPTS_FOLDER.to_str().unwrap_or_default().to_string();

                FileUtils::create_path(&path);
                Remote::download(&line_trimmed, &path).await?
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
