use crate::{
    utils::str::StrUtils,

    consts::{
        uris::Uris,
        global::Global,
    },
};

pub struct PrimeDownInject;

impl PrimeDownInject {

    pub fn content(file: &str, contents: String, markdown_html: String) -> String {
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
        )
    }

    pub fn html_content(css_content: String, html_content: String) -> String {
        format!(
            "<!DOCTYPE html>
            <html lang=\"en\">
            <head>
                <meta charset=\"UTF-8\">
                <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
                <style>{}</style>
            </head>
            <body>
                <article class=\"markdown-body\">{}</article>
            </body>
            </html>",
            css_content, html_content
        )
    }
    
}
