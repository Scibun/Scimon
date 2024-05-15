use crate::{
    system::markdown::Markdown,

    render::{
        render_io::RenderMarkdownIO,
        render_utils::RenderMarkdownUtils,
    }
};

pub struct RenderMarkdown;

impl RenderMarkdown {

    pub fn render_and_save_file(file: &str, no_open_link: bool) {
        if let Some(markdown_html) = Markdown::render_readme(file) {
            let path = RenderMarkdownIO::get_file_path(file);
            let contents = RenderMarkdownUtils::render_content(&file, markdown_html);

            RenderMarkdownIO::write_file(&path, contents);
            RenderMarkdownIO::open_readme_url(&path, no_open_link)
        }
    }
    
}
