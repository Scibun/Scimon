use std::{
    io,
    error::Error,
    
    process::{
        Stdio,
        Command,
    },
};

use crate::{
    ui::{
        ui_base::UI,
        pip_alerts::PipAlerts,
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

    fn check_pip_package(package: &str) -> Result<bool, io::Error> {
        let output = Command::new("python")
            .arg("-c")
            .arg(format!("import {}; print('exists')", package))
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .output()?;
    
        Ok(output.stdout.contains(&b"exists"[0]))
    }

    async fn exec(path: &str, silent_mode: bool) -> Result<(), Box<dyn Error>> {
        let output = Command::new("pip")
            .arg("install")
            .arg("-r")
            .arg("requirements.txt")
            .current_dir(path)
            .stdout(Stdio::piped())
            .output()?;
        
        if !silent_mode {
            if output.status.success() {
                PipAlerts::success();
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                PipAlerts::error_install(&stderr);
            }
        }

        Ok(())
    }

    pub async fn check_pip_packages() -> Result<(), Box<dyn Error>> {
        let mut need_install = false;
        let content = Remote::content(Addons::INSTALL_REQUIREMENTS_PLUGINS).await?;
    
        for line in content.lines() {
            let package = line.split("==").next().unwrap_or("").trim();
            if package.is_empty() { continue; }
    
            match Self::check_pip_package(package) {
                Ok(true) => need_install = false,
                Ok(false) => need_install = true,
                Err(_) => PipAlerts::error_check(package),
            }
        }

        if need_install { Self::install(true).await?; }
        Ok(())
    }    

    pub async fn install(silent_mode: bool) -> Result<(), Box<dyn Error>> {
        if !silent_mode {
            UI::section_header("Installing requirements", "info");
        }

        let path = Folders::SCRIPTS_FOLDER.to_str().unwrap_or_default().to_string();

        FileUtils::create_path(&path);
        Remote::download(&Addons::INSTALL_REQUIREMENTS_PLUGINS, &path).await?;

        Self::exec(&path, silent_mode).await?;
        Ok(())
    }

}