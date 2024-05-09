extern crate url;

use url::Url;
use regex::Regex;

use crate::configs::regex::RegExp;

pub struct UrlMisc;

impl UrlMisc {

    pub fn get_domain(url: &str) -> String {
        let url = Url::parse(url).expect("");
        url.host_str().expect("").to_owned()
    }

    // pub fn get_protocol(url: &str) -> String {
    //     let url = Url::parse(url).expect("Failed to parse URL");
    //     url.scheme().to_string()
    // }

    pub fn extract_url(line: &str) -> String {
        let re = Regex::new(RegExp::EXTRACT_URL).unwrap();

        if let Some(capture) = re.find(line) {
            capture.as_str().to_string()
        } else {
            String::new()
        }
    }

    pub fn escape_quotes(url: &str) -> String {
        url.replace("\"", "%22")
    }

    pub fn get_subdomain(url: &str) -> String {
        let url = Url::parse(url).expect("");
        let host = url.host_str().expect("");
        host.split('.').next().expect("").to_owned()
    }
 
    pub fn get_last_part(line: &str) -> String {
        let parts: Vec<&str> = line.split('/').collect();

        if let Some(last_part) = parts.last() {
            last_part.to_string()
        } else {
            String::new()
        }
    }

    pub fn open_url(url: &str, extract_url: bool) {
        let open_url = if extract_url {
            Self::extract_url(url)
        } else {
            url.to_string()
        };

        let _ = webbrowser::open(&open_url);
    }

    pub async fn get_status_code(url: &str) -> u16 {
        reqwest::get(url)
            .await
            .map(|response| response.status().as_u16())
            .unwrap_or(0)
    }

    pub fn check_domain(line: &str, domain: &str) -> bool {
        line.contains(domain)
    }

}
