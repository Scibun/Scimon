use std::fs;
use regex::Regex;
use serde_yaml::Value;

use crate::{
    configs::settings::Settings,

    render::{
        render_env::RenderMarkdownEnv,
        injection::render_inject_regex::RenderMarkdownInjectRegExp,
    },
};

pub struct RenderMarkdownInjectJS;

impl RenderMarkdownInjectJS {

    fn minify(code: &str) -> String {
        let code = Regex::new(RenderMarkdownInjectRegExp::MIN_JS_REMOVE_WHITESPACE).unwrap().replace_all(code, " ");
        let code = Regex::new(RenderMarkdownInjectRegExp::MIN_JS_REMOVE_SINGLE_LINE_COMMENT).unwrap().replace_all(&code, "");
        let code = Regex::new(RenderMarkdownInjectRegExp::MIN_JS_REMOVE_MULTI_LINE_COMMENT).unwrap().replace_all(&code.trim(), "");
        let code = Regex::new(RenderMarkdownInjectRegExp::MIN_JS_REMOVE_STRINGS).unwrap().replace_all(&code, "\"\"");
        let code = Regex::new(RenderMarkdownInjectRegExp::MIN_JS_REMOVE_OPERATORS_KEYWORDS).unwrap().replace_all(&code, "$1");
        let code = Regex::new(RenderMarkdownInjectRegExp::MIN_JS_REMOVE_SPACES).unwrap().replace_all(&code, "$1");

        let code = Regex::new(
            &format!(r"\b({})\b", RenderMarkdownInjectRegExp::MIN_JS_KEYWORDS)
        ).unwrap().replace_all(
            &code, " $1 "
        );

        let code = Regex::new(RenderMarkdownInjectRegExp::MIN_JS_DUPLICATE_SPACES).unwrap().replace_all(&code, " ");
        let code = Regex::new(RenderMarkdownInjectRegExp::MIN_JS_LOGICAL_OPERATORS).unwrap().replace_all(&code, "||");

        let code = code.replace(
            "; ", ";"
        ).replace(
            "if (", "if("
        ).replace(
            " + ", "+"
        ).replace(
            "( ", "("
        );

        let code = Regex::new(RenderMarkdownInjectRegExp::MIN_JS_DOUBLE_QUOTED_STRING).unwrap().replace_all(
            &code, |caps: &regex::Captures| {
                let inner = &caps[0][1..caps[0].len() - 1];
                format!("\"{}\"", inner.replace("\\\"", "\""))
            }
        );

        code.to_string()
    }
    
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
        
        content_js = Self::minify(&content_js);
        format!("<script>{}</script>", &content_js)
    }

}
