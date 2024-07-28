use walkdir::WalkDir;

use std::{
    path::Path,
    error::Error,
    io::Result as IoResult,
};

use pyo3::{
    prelude::*,
    types::PyModule,
};

use crate::{
    syntax::vars::Vars,

    consts::{
        addons::Addons,
        folders::Folders,
    },

    ui::{
        ui_base::UI,
        success_alerts::SuccessAlerts,
    },
    
    utils::{
        remote::Remote,
        file::FileUtils,
    },
};

pub struct ExtractCovers;

impl ExtractCovers {

    async fn exec(input_file: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
        let script_path = &format!("{}", Folders::PLUGINS_FOLDER.to_str().unwrap_or_default());
        FileUtils::create_path(&script_path);

        Remote::download(Addons::EXTRACT_COVERS_PLUGIN, script_path).await?;
        
        let plugin_path = &format!("{}/extract_cover.py", script_path);
    
        Python::with_gil(|py| {
            let code = std::fs::read_to_string(plugin_path)?;
            let main_ctx = PyModule::from_code_bound(py, &code, script_path, script_path)?;
    
            let file: String = main_ctx
                .getattr("extract_first_page_to_png")?
                .call1((input_file, output_path))?
                .extract()?;
    
            SuccessAlerts::cover_generated(file.as_str());
            Ok(())
        })
    }

    pub async fn extract(contents: &str) -> IoResult<()> {
        if let Some(covers_path) = Vars::get_covers(contents) {
            UI::section_header("Extracting covers", "normal");
            let pdf_path = &Vars::get_path(contents);

            for entry in WalkDir::new(&pdf_path) {
                let entry = entry?;
                let path = entry.path();
                let file_name = path.strip_prefix(Path::new(&pdf_path)).unwrap();

                let new_name = FileUtils::replace_extension(file_name.to_str().unwrap_or_default(), "png");
                let input_file = &format!("{}{}", pdf_path, file_name.to_str().unwrap_or_default());
                let output_path = &format!("{}{}", covers_path, new_name);

                let _ = Self::exec(input_file, output_path).await;
            }
        }

        Ok(())
    }

}
