use std::fs;
use serde_yaml::Value;

use crate::{
    configs::settings::Settings,

    render::{
        render_env::RenderMarkdownEnv,
        injection::render_inject_minify::RenderMarkdownInjectMinify,
    },
};

pub struct RenderMarkdownInjectJS;

impl RenderMarkdownInjectJS {
    
    fn generate_script_tags(css_list: &[Value]) -> String {
        let mut tags = String::new();

        for css_file in css_list {
            if let Value::String(file_name) = css_file {
                tags.push_str(&format!("<script src=\"{}\"></script>\n", file_name));
            }
        }
    
        tags
    }

    fn from_cdn() -> Value {
        let js_list = Settings::get("render_markdown.load_js_cdn", "LIST");

        if let Value::Sequence(js_list) = js_list {
            Value::String(Self::generate_script_tags(&js_list))
        } else {
            Value::Null
        }
    }

    pub fn load_from_cdn() -> String {
        serde_yaml::to_string(
            &Self::from_cdn()
        ).unwrap_or_default().replace(
            "|\n", "\n"
        ).trim().to_string()
    } 

    pub fn load_from_files() -> String {
        let mut content_js = String::new();
        let js_path = RenderMarkdownEnv::README_TEMPLATE_JS_FILES;
    
        for entry in fs::read_dir(js_path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
    
            if path.is_file() && path.extension().map_or(false, |ext| ext == "js") {
                let js_content = fs::read_to_string(path).unwrap();
                content_js.push_str(
                    &format!("{}\n", &js_content)
                );
            }
        }
        
        content_js = RenderMarkdownInjectMinify::js(&content_js);
        format!("<script>{}</script>", &content_js)
    }

}
