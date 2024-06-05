use serde_yaml::Value;

use crate::{
    utils::str::StrUtils,
    consts::global::Global,
    configs::settings::Settings,

    prime_down::inject::{
        pd_inject_js::PrimeDownInjectJS,
        pd_inject_css::PrimeDownInjectCSS,
    }
};

pub struct PrimeDownInject;

impl PrimeDownInject {

    fn get_js(render_mode: Value, minify: Value) -> String {
        let cdn = if render_mode == Global::APP_NAME {
            PrimeDownInjectJS::load_from_cdn()
        } else {
            "".to_string()
        };

        let local = if render_mode == Global::APP_NAME {
            PrimeDownInjectJS::load_from_files(minify)
        } else {
            "".to_string()
        };

        format!(
            "{}{}", cdn, local
        )
    }

    fn get_css(render_mode: Value, minify: Value) -> String {
        let local = if render_mode == Global::APP_NAME {
            PrimeDownInjectCSS::load_from_files(minify)
        } else {
            "".to_string()
        };

        format!("{}", local)
    }

    pub fn content(file: &str, contents: String, markdown_html: String) -> String {
        let render_mode = Settings::get("render_markdown.mode", "STRING");
        let render_minify_extra_plugins = Settings::get("render_markdown.minify_extra_plugins", "BOOLEAN");

        let title = format!(
            "{}: {}: README", StrUtils::capitalize(&Global::APP_NAME), &file.replace(
                ".md", ""
            )
        );
        
        contents.replace(
            "{{ page_title }}", &title
        ).replace(
            "{{ markdown_content }}", &markdown_html
        ).replace(
            "{{ inject_css }}", &Self::get_css(
                render_mode.clone(), render_minify_extra_plugins.clone()
            )
        ).replace(
            "{{ inject_js }}", &Self::get_js(
                render_mode.clone(), render_minify_extra_plugins.clone()
            )
        )
    }
    
}
