use regex::Regex;

use crate::{
    generator::generate::Generate,
    regexp::regex_core::CoreRegExp,
};

pub struct PrimeDownExtras;

impl PrimeDownExtras {
   
    pub fn gist(markdown: &str) -> String {
        let re = Regex::new(CoreRegExp::RENDER_EXTRA_GIST).unwrap();
    
        let replaced_markdown = re.replace_all(markdown, |captures: &regex::Captures| {
            let link = captures.get(1).unwrap().as_str();
            format!("<script src='{}.js'></script>", link)
        });
    
        replaced_markdown.to_string()
    }
   
    pub fn qrcode(markdown: &str) -> String {
        let re = Regex::new(CoreRegExp::RENDER_EXTRA_QRCODE).unwrap();
    
        let replaced_markdown = re.replace_all(markdown, |captures: &regex::Captures| {
            let link = captures.get(1).unwrap().as_str();
            let size: u32 = captures.get(2).unwrap().as_str().parse().unwrap();
    
            let qr_code_base64 = Generate::qrcode(link, size);
            let link_qrcode = format!("data:image/png;base64,{}", qr_code_base64);

            format!(
                "<p class='qrcode'>
                    <img src='{}' alt='QR Code of {}' />
                </p>", link_qrcode, link
            )
        });
    
        replaced_markdown.to_string()
    }
    
}
