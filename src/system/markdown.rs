extern crate open;

use std::{
    env,
    error::Error,
};

use pulldown_cmark::{
    html,
    Parser,
    Options,
};

use crate::{
    system::pdf::Pdf, 
    cmd::tasks::Tasks,
    configs::settings::Settings, 
    generator::generate::Generate,
    ui::success_alerts::SuccessAlerts, 
    
    render::{
        render_io::RenderIO,
        render_inject::RenderInject,
        render_extras::RenderExtras, 
    }, 
    
    utils::{
        url::UrlMisc,
        remote::Remote, 
        file::FileUtils,
    }
};

pub struct Markdown;

impl Markdown {

    pub fn open_file(path: &str, no_open_link: bool) {
        if !no_open_link {
            let full_path = env::current_dir().expect(
                ""
            ).join(&path).to_str().unwrap().replace(
                "\\", "/"
            );

            let url_file = &format!(
                "file://{}", full_path
            );

            let _ = open::that(&url_file);
        }
    }

    pub fn get_filename_rendered(file: &str) -> String {
        let filename = if Settings::get("render_markdown.overwrite", "BOOLEAN") == true {
            ".html".to_string()
        } else {
            format!(
                "_{}.html", Generate::random_string(16)
            )
        };

        RenderIO::get_file_path(file).replace(
            ".html", &filename
        )
    }

    pub fn append_extras_and_render(markdown: &str) -> String {
        let markdown_block_extras_qrcode = RenderExtras::qrcode(&markdown);
        let markdown_block_extras_gist = RenderExtras::gist(&markdown_block_extras_qrcode);

        let parser = Parser::new_ext(&markdown_block_extras_gist, Options::all());
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        format!("<div class=\"markdown-content\">{}</div>", html_output)
    }

    pub async fn render(url: &str) -> Result<String, Box<dyn Error>> {
        let markdown_content = Remote::content(url).await?;
    
        let options = Options::empty();
        let parser = Parser::new_ext(&markdown_content, options);

        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
    
        Ok(html_output)
    }

    pub async fn create(contents: &str, url: &str, path: &str) -> Result<(), Box<dyn Error>> {
        if Remote::check_content_type(&url, "text/markdown").await? || url.contains(".md") {
            let html_content = Self::render(url).await?;
            let content = RenderInject::html_content(contents, html_content).await?;
            
            let original_name = UrlMisc::get_last_part(url);
            let new_filename = FileUtils::replace_extension(&original_name, "pdf");
            let output_path = FileUtils::get_output_path(&path, &new_filename);
            let output_path_str = format!("{}{}", &path, &new_filename);

            Pdf::create_pdf(&content, output_path, &url).await?;

            let hash = Tasks::hash_sha256(&output_path_str)?;
            SuccessAlerts::download_and_generated_pdf(&new_filename, url, &hash);
        }

        Ok(())
    }

}
