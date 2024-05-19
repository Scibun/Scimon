use crate::{
    configs::settings::Settings,
    ui::macros_alerts::MacrosAlerts,

    utils::{
        file::FileMisc,
        generate::Generate,
    },

    prime_down::{
        pd_core::PrimeDown,
        pd_io::PrimeDownIO,
        pd_misc::PrimeDownMisc,
    }
};

pub struct ReadMe;

impl ReadMe {

    pub fn render_and_save_file(file: &str, no_open_link: bool) {
        if let Some(markdown_html) = PrimeDown::render_readme(file) {
            let filename = if Settings::get("render_markdown.overwrite", "BOOLEAN") == true {
                ".html".to_string()
            } else {
                format!(
                    "_{}.html", Generate::random_string(16)
                )
            };

            let path = PrimeDownIO::get_file_path(file).replace(
                ".html", &filename
            );

            let contents = PrimeDownMisc::render_content(&file, markdown_html);

            FileMisc::write_file(&path, contents);
            PrimeDownMisc::open_readme_url(&path, no_open_link);
            
            MacrosAlerts::readme(&path);
        }
    }

}
