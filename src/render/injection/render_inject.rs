use crate::render::{
    render_env::RenderMarkdownEnv,

    injection::{
        render_inject_js::RenderMarkdownInjectJS,
        render_inject_css::RenderMarkdownInjectCSS,
    },
};

pub struct RenderMarkdownInject;

impl RenderMarkdownInject {

    pub fn content(file: &str, contents: String, markdown_html: String) -> String {
        let js_files_string = &RenderMarkdownInjectJS::load_from_files();
        let js_cdn_files_string = &RenderMarkdownInjectJS::load_from_cdn();

        let css_files_string = &RenderMarkdownInjectCSS::load_from_files();
        let css_cdn_files_string = &RenderMarkdownInjectCSS::load_from_cdn();

        let title = format!("{}: {}: README", &RenderMarkdownEnv::README_APP_NAME, &file);
        
        contents.replace(
            "{{ page_title }}", &title
        ).replace(
            "{{ inject_css_cdn }}", &css_cdn_files_string
        ).replace(
            "{{ inject_css_files }}", &css_files_string
        ).replace(
            "{{ markdown_content }}", &markdown_html
        ).replace(
            "{{ inject_js_cdn }}", &js_cdn_files_string
        ).replace(
            "{{ inject_js_files }}", &js_files_string
        )
    }
    
}
