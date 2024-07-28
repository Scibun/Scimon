use walkdir::WalkDir;

use std::{
    path::Path,
    error::Error,
};

use pyo3::{
    prelude::*,
    types::PyModule,
};

use crate::{
    syntax::vars::Vars,
    consts::addons::Addons,

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

    async fn exec(code: &str, input_file: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
        Python::with_gil(|py| {
            let main_ctx = PyModule::from_code_bound(py, &code, "", "")?;
    
            let file: String = main_ctx
                .getattr("extract_first_page_to_png")?
                .call1((input_file, output_path))?
                .extract()?;
    
            SuccessAlerts::cover_generated(file.as_str());
            Ok(())
        })
    }

    pub async fn extract(contents: &str) -> Result<(), Box<dyn Error>> {
        if let Some(covers_path) = Vars::get_covers(contents) {
            FileUtils::create_path(&covers_path);
            let code = &Remote::content(Addons::EXTRACT_COVERS_PLUGIN).await?;

            UI::section_header("Extracting covers", "normal");
            let pdf_path = &Vars::get_path(contents);

            for entry in WalkDir::new(&pdf_path) {
                let entry = entry?;
                let path = entry.path();
                let file_name = path.strip_prefix(Path::new(&pdf_path)).unwrap();

                let new_name = FileUtils::replace_extension(file_name.to_str().unwrap_or_default(), "png");
                let input_file = &format!("{}{}", pdf_path, file_name.to_str().unwrap_or_default());
                let output_path = &format!("{}{}", covers_path, new_name);

                let _ = Self::exec(code, input_file, output_path).await;
            }
        }

        Ok(())
    }

}
