use crate::{
    consts::uris::Uris,
    utils::domains::Domains,
};

pub struct RenderName {
    pub domain: String,
    pub url_slices: Vec<String>,
}

impl RenderName {

    pub fn new(url: &str) -> Self {
        Self {
            domain: Domains::get(url),
            url_slices: url.split("/").map(|s| s.to_string()).collect(),
        }
    }

    fn last_part(&self, slice: usize) -> String {
        let segments = &self.url_slices;
        let url_slice = &segments[segments.len() - slice];
        url_slice.to_string()
    }

    pub fn get_filename(&self) -> String {
        match &self.domain {
            domain if domain == Uris::PROVIDERS_DOMAINS[2] => self.last_part(3), // github,
            domain if domain == Uris::PROVIDERS_DOMAINS[3] => self.last_part(5), // gitlab,
            domain if domain == Uris::PROVIDERS_DOMAINS[4] => self.last_part(4), // bitbucket,
            domain if domain == Uris::PROVIDERS_DOMAINS[5] => self.last_part(5), // codeberg,
            _ => self.url_slices.last().unwrap().to_string(),
        }
    }

}
