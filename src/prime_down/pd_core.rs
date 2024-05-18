extern crate colored;

use crate::prime_down::{
    pd_io::PrimeDownIO,
    pd_misc::PrimeDownMisc,
    pd_engine::PrimeDownEngine,
};

use crate::utils::file::FileMisc;

pub struct PrimeDown;

impl PrimeDown {

    pub fn render_and_save_file(file: &str, no_open_link: bool) {
        if let Some(markdown_html) = PrimeDownEngine::render_readme(file) {
            let path = PrimeDownIO::get_file_path(file);
            let contents = PrimeDownMisc::render_content(&file, markdown_html);

            FileMisc::write_file(&path, contents);
            PrimeDownMisc::open_readme_url(&path, no_open_link)
        }
    }

}
