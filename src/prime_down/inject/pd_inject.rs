use serde_yaml::Value;

use crate::{
    utils::str::StrUtils,
    configs::settings::Settings,

    consts::{
        uris::Uris,
        global::Global,
    },

    prime_down::inject::pd_inject_js::PrimeDownInjectJS,
};

pub struct PrimeDownInject;

impl PrimeDownInject {

    fn get_js(render_mode: Value) -> String {
        let cdn = if render_mode == Global::APP_NAME {
            PrimeDownInjectJS::load_from_cdn()
        } else {
            "".to_string()
        };

        format!("{}", cdn)
    }

    pub fn content(file: &str, contents: String, markdown_html: String) -> String {
        let render_mode = Settings::get("render_markdown.mode", "STRING");

        let title = format!(
            "{}: {}: README", StrUtils::capitalize(&Global::APP_NAME), &file.replace(
                ".md", ""
            )
        );

        let bundle_js_link = format!("{}static/dist/bundle.js", Uris::README_TEMPLATE_LINK);
        let bundle_css_link = format!("{}static/dist/bundle.css", Uris::README_TEMPLATE_LINK);
        
        contents.replace(
            "{{ page_title }}", &title
        ).replace(
            "{{ dist_bundle_css }}", &bundle_css_link
        ).replace(
            "{{ dist_bundle_js }}", &bundle_js_link
        ).replace(
            "{{ markdown_content }}", &markdown_html
        ).replace(
            "{{ inject_js }}", &Self::get_js(render_mode.clone())
        )
    }
    
}
