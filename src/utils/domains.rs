extern crate url;

use url::Url;

pub struct Domains;

impl Domains {

    pub fn get(url: &str) -> String {
        let url = Url::parse(url).expect("");
        url.host_str().expect("").to_owned()
    }

    pub fn subdomain(url: &str) -> String {
        let url = Url::parse(url).expect("");
        let host = url.host_str().expect("");
        host.split('.').next().expect("").to_owned()
    }

    pub fn check(url: &str, domain: &str) -> bool {
        url.contains(domain)
    }

}
