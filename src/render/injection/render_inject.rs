use serde_yaml::Value;

use crate::{
    consts::global::Global,
    configs::settings::Settings,

    render::injection::{
        render_inject_js::RenderMarkdownInjectJS,
        render_inject_css::RenderMarkdownInjectCSS,
    }
};

pub struct RenderMarkdownInject;

impl RenderMarkdownInject {

    fn get_js(render_mode: Value, minify: Value) -> String {
        let cdn = if render_mode == "paimon" {
            RenderMarkdownInjectJS::load_from_cdn()
        } else {
            "".to_string()
        };

        let local = if render_mode == "paimon" {
            RenderMarkdownInjectJS::load_from_files(minify)
        } else {
            "".to_string()
        };

        format!("{}{}", cdn, local)
    }

    fn get_css(render_mode: Value, minify: Value) -> String {
        let cdn = if render_mode == "paimon" {
            RenderMarkdownInjectCSS::load_from_cdn()
        } else {
            "".to_string()
        };

        let local = if render_mode == "paimon" {
            RenderMarkdownInjectCSS::load_from_files(minify)
        } else {
            "".to_string()
        };

        format!("{}{}", cdn, local)
    }

    pub fn content(file: &str, contents: String, markdown_html: String) -> String {
        let render_mode = Settings::get("render_markdown.mode", "STRING");
        let render_minify_extra_plugins = Settings::get("render_markdown.minify_extra_plugins", "BOOLEAN");

        let title = format!("{}: {}: README", &Global::APP_NAME, &file);
        
        contents.replace(
            "{{ page_title }}", &title
        ).replace(
            "{{ markdown_content }}", &markdown_html
        ).replace(
            "{{ inject_css }}", &Self::get_css(render_mode.clone(), render_minify_extra_plugins.clone())
        ).replace(
            "{{ inject_js }}", &Self::get_js(render_mode.clone(), render_minify_extra_plugins.clone())
        )
    }
    
}
