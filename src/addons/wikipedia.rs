use crate::{
    consts::uris::Uris,
    
    utils::{
        url::UrlMisc,
        domain::Domain,
    },
};

pub struct Wikipedia;

impl Wikipedia {

    pub fn wikipedia(url: &str) -> (String, String) {
        let wiki_name = UrlMisc::get_last_part(url);
        let wikipedia_region = format!("{}.", Domain::new(url).subdomain());

        let request_url = format!("{}{}", Uris::WIKIPEDIA_API_REQUEST_PDF.to_string().replace(
            "en.", &wikipedia_region
        ), wiki_name);

        let filename = format!("{}.pdf", wiki_name);

        (request_url, filename)
    }

    pub fn wikisource(url: &str) -> (String, String) {
        let wiki_name = UrlMisc::get_last_part(url);
        let wikipedia_region = format!("{}.", Domain::new(url).subdomain());

        let request_url = format!("{}{}", Uris::WIKISOURCE_API_REQUEST_PDF.to_string().replace(
            "en.", &wikipedia_region
        ), wiki_name);

        let filename = format!("{}.pdf", wiki_name);

        (request_url, filename)
    }

}
