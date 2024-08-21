extern crate url;

use url::Url;

pub struct Domain {
    url: String,
}

impl Domain {

    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_owned(),
        }
    }

    pub fn get(&self) -> String {
        let url = Url::parse(&self.url).expect("");
        url.host_str().expect("").to_owned()
    }

    pub fn subdomain(&self) -> String {
        let url = Url::parse(&self.url).expect("");
        let host = url.host_str().expect("");
        host.split('.').next().expect("").to_owned()
    }

    pub fn check(&self, domain: &str) -> bool {
        self.url.contains(domain)
    }

}
