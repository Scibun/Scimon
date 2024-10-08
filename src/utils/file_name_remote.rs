use crate::{
    consts::uris::Uris,
    utils::domain::Domain,
};

pub struct FileNameRemote {
    domain: String,
    url_slices: Vec<String>,
}

impl FileNameRemote {

    pub fn new(url: &str) -> Self {
        Self {
            domain: Domain::new(url).get(),
            url_slices: url.split("/").map(|s| s.to_string()).collect(),
        }
    }

    fn slice(&self, slice: usize) -> String {
        let segments = &self.url_slices;
        let url_slice = &segments[segments.len() - slice];
        url_slice.to_string()
    }

    pub fn get(&self) -> String {
        match &self.domain {
            domain if domain == Uris::PROVIDERS_DOMAINS[2] => self.slice(3), // github
            domain if domain == Uris::PROVIDERS_DOMAINS[3] => self.slice(5), // gitlab
            domain if domain == Uris::PROVIDERS_DOMAINS[4] => self.slice(4), // bitbucket
            domain if domain == Uris::PROVIDERS_DOMAINS[5] => self.slice(5), // codeberg
            _ => self.url_slices.last().unwrap().to_string(),
        }
    }

}
