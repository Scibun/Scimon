use reqwest::Client;

use select::{
    predicate::Name,
    document::Document,
};

pub struct SciHub;

impl SciHub {

    fn extract_src_from_embed(html: &str) -> Option<String> {
        let document = Document::from(html);
    
        if let Some(embed) = document.find(Name("embed")).next() {
            if let Some(src) = embed.attr("src") {
                return Some(src.to_string());
            }
        }
    
        None
    }

    fn extract_pdf_url(url: &str) -> Option<String> {
        if let Some(index) = url.find(".pdf") {
            let pdf_url = &url[..=index+3];
            return Some(pdf_url.to_string());
        }

        None
    }
    
    pub async fn get_pdf_file(url: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut pdf_url = String::new();

        let response = Client::new().get(url).send().await?;
        let body = response.text().await?;

        for line in body.lines() {
            let line_trim = line.trim();

            if line_trim.starts_with("<embed type=") {
                if let Some(src) = Self::extract_src_from_embed(line_trim) {
                    let get_src = if src.starts_with("//") {
                        src.replace("//", "https://")
                    } else {
                        src
                    };

                    if let Some(final_pdf_url) = Self::extract_pdf_url(&get_src) {
                        pdf_url = final_pdf_url;
                        break;
                    } else {
                        pdf_url = "".to_string();
                    }
                } else {
                    pdf_url = "".to_string();
                }
            }
        }

        Ok(pdf_url)
    }

}
