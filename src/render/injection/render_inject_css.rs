use std::fs;
use serde_yaml::Value;

use crate::{
    configs::settings::Settings,
    consts::render::RenderMarkdownEnv,
    render::render_minify::RenderMarkdownMinify,
};

pub struct RenderMarkdownInjectCSS;

impl RenderMarkdownInjectCSS {
    
    fn generate_link_tags(css_list: &[Value]) -> String {
        let mut tags = String::new();

        for css_file in css_list {
            if let Value::String(file_name) = css_file {
                tags.push_str(&format!("<link rel=\"stylesheet\" href=\"{}\">\n", file_name));
            }
        }
    
        tags
    }

    fn from_cdn() -> Value {
        let css_list = Settings::get("render_markdown.load_css_cdn", "LIST");

        if let Value::Sequence(css_list) = css_list {
            Value::String(Self::generate_link_tags(&css_list))
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

    pub fn load_from_files(minify: Value) -> String {
        let mut content_css = String::new();
        let js_path = RenderMarkdownEnv::README_TEMPLATE_CSS_FILES;
    
        for entry in fs::read_dir(js_path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
    
            if path.is_file() && path.extension().map_or(false, |ext| ext == "css") {
                let js_content = fs::read_to_string(path).unwrap();
                content_css.push_str(
                    &format!("{}\n", &js_content)
                );
            }
        }
    
        content_css = if minify == true {
            RenderMarkdownMinify::css(&content_css)
        } else {
            content_css
        };

        format!("<style>{}</style>", &content_css)
    }
    
}
