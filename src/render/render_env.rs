use std::path::PathBuf;
use once_cell::sync::Lazy;

use crate::configs::global::Global;

pub struct RenderEnv;

impl RenderEnv {

    pub const README_TEMPLATE_FILE: &'static str = "templates/readme.html";
    pub const README_CSS_BASE_FILE: &'static str = "https://cdnjs.cloudflare.com/ajax/libs/github-markdown-css/5.5.1/github-markdown-dark.min.css";

    pub const README_FOLDER: Lazy<PathBuf> = Lazy::new(|| {
        let mut path = Global::APP_FOLDER.clone();
        path.push("readme");
        path
    });

}
