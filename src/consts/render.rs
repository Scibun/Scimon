use crate::consts::global::Global;

pub struct RenderMarkdownEnv;

impl RenderMarkdownEnv {

    pub const README_APP_NAME: &'static str = Global::APP_NAME;
    pub const README_TEMPLATE_JS_FILES: &'static str = "static/js/";
    pub const README_TEMPLATE_CSS_FILES: &'static str = "static/css/";
    pub const README_TEMPLATE_FILE: &'static str = "static/templates/render.html";

}
