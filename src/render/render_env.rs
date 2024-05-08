use std::path::PathBuf;
use once_cell::sync::Lazy;

use crate::configs::global::Global;

pub struct RenderMarkdownEnv;

impl RenderMarkdownEnv {

    pub const README_APP_NAME: &'static str = Global::APP_NAME;
    pub const README_TEMPLATE_JS_FILES: &'static str = "static/js/";
    pub const README_TEMPLATE_CSS_FILES: &'static str = "static/css/";
    pub const README_TEMPLATE_FILE: &'static str = "templates/render.html";

    pub const README_FOLDER: Lazy<PathBuf> = Lazy::new(|| {
        let mut path = Global::APP_FOLDER.clone();
        path.push("readme");
        path
    });

}
