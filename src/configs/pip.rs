use std::{
    error::Error,
    
    process::{
        Stdio,
        Command,
    },
};

use crate::{
    ui::{
        ui_base::UI,
        success_alerts::SuccessAlerts,
    },

    consts::{
        addons::Addons,
        folders::Folders,
    },

    utils::{
        remote::Remote,
        file::FileUtils,
    },
};

pub struct Pip;

impl Pip {

    async fn exec(path: &str) -> Result<(), Box<dyn Error>> {
        let output = Command::new("pip")
            .arg("install")
            .arg("-r")
            .arg("requirements.txt")
            .current_dir(path)
            .stdout(Stdio::piped())
            .output()?;
        
        if output.status.success() {
            let _ = String::from_utf8_lossy(&output.stdout);
            SuccessAlerts::pip();
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("{}", stderr);
        }

        Ok(())
    }

    pub async fn install() -> Result<(), Box<dyn Error>> {
        UI::section_header("Installing requirements", "info");
        let path = Folders::SCRIPTS_FOLDER.to_str().unwrap_or_default().to_string();

        FileUtils::create_path(&path);
        Remote::download(&Addons::INSTALL_REQUIREMENTS_PLUGINS, &path).await?;

        Self::exec(&path).await?;
        Ok(())
    }

}