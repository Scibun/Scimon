extern crate colored;

use crate::prime_down::{
    pd_io::PrimeDownIO,
    pd_misc::PrimeDownMisc,
    pd_core::PrimeDown,
};

use crate::utils::file::FileMisc;

pub struct PrimeDownRender;

impl PrimeDownRender {

    pub fn render_and_save_file(file: &str, no_open_link: bool) {
        if let Some(markdown_html) = PrimeDown::render_readme(file) {
            let path = PrimeDownIO::get_file_path(file);
            let contents = PrimeDownMisc::render_content(&file, markdown_html);

            FileMisc::write_file(&path, contents);
            PrimeDownMisc::open_readme_url(&path, no_open_link)
        }
    }

}
