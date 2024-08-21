use std::error::Error;

use crate::{
    consts::uris::Uris,
    addons::wikipedia::Wikipedia,
    
    utils::{
        url::UrlMisc,
        remote::Remote,
        domain::Domain,
    },
};

pub struct Providers;

impl Providers {

    pub fn arxiv(url: &str) -> String {
        let escape_quotes = UrlMisc::escape_quotes(url);
        let domain = Domain::new(url);

        if !domain.check(Uris::PROVIDERS_DOMAINS[1]) {
            escape_quotes.to_owned()
        } else {
            escape_quotes.replace("/abs/", "/pdf/")
        }
    }

    pub fn check_provider_domain(url: &str) -> bool {
        let mut valid_domain = false;
        let domain = Domain::new(url);

        for url_domain in &Uris::PROVIDERS_DOMAINS {
            if domain.check(url_domain) {
                valid_domain = true
            }
        }

        valid_domain
    }

    pub async fn generic(url: &str) -> Result<(String, String), Box<dyn Error>> {
        let request_uri = url.to_string();
        let filename = Remote::get_filename(url, true).await?;

        Ok((request_uri, filename))
    }

    pub async fn get_from_provider(url: &str) -> Result<(String, String), Box<dyn Error>> {
        let filename;
        let request_uri;
        let domain = Domain::new(url);

        if domain.check(Uris::PROVIDERS_DOMAINS[0]) {
            (request_uri, filename) = Wikipedia::wikipedia(url);
        } else if domain.check(Uris::PROVIDERS_DOMAINS[1]) {
            (request_uri, filename) = Wikipedia::wikisource(url);
        } else {
            (request_uri, filename) = Self::generic(url).await?;
        }

        Ok((request_uri, filename))
    }

}
