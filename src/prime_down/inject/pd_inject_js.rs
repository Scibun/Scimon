use serde_yaml::Value;

use crate::configs::settings::Settings;

pub struct PrimeDownInjectJS;

impl PrimeDownInjectJS {
    
    fn generate_script_tags(css_list: &[Value]) -> String {
        let mut tags = String::new();

        for css_file in css_list {
            if let Value::String(file_name) = css_file {
                let script_js = &format!(
                    "<script src=\"{}\"></script>\n", file_name
                );

                tags.push_str(&script_js);
            }
        }
    
        tags
    }

    fn from_cdn() -> Value {
        let js_list = Settings::get("render_markdown.load_js_cdn", "LIST");

        if let Value::Sequence(js_list) = js_list {
            Value::String(
                Self::generate_script_tags(&js_list)
            )
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

}
