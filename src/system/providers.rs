use std::error::Error;

use crate::{
    consts::uris::Uris,
    addons::wikipedia::Wikipedia,
    
    utils::{
        url::UrlMisc,
        domains::Domains,
        remote::FileRemote,
    },
};

pub struct Providers;

impl Providers {

    fn extract_doi(url: &str) -> String {
        if let Some(index) = url.find('/') {
            let restante = &url[index + 2..];
            
            if let Some(index) = restante.find('/') {
                return restante[index + 1..].to_string();
            }
        }

        String::new()
    }

    pub fn arxiv(url: &str) -> String {
        let escape_quotes = UrlMisc::escape_quotes(url);

        if !Domains::check(&escape_quotes, Uris::PROVIDERS_DOMAINS[1]) {
            escape_quotes.to_owned()
        } else {
            escape_quotes.replace("/abs/", "/pdf/")
        }
    }
    
    pub fn github(url: &str) -> String {
        let escape_quotes = UrlMisc::escape_quotes(url);

        if !Domains::check(&escape_quotes, Uris::PROVIDERS_DOMAINS[2]) {
            escape_quotes.to_owned()
        } else if !Domains::check(&escape_quotes, Uris::PROVIDERS_DOMAINS[3]) {
            escape_quotes.to_owned()
        } else {
            escape_quotes.replace("/blob/", "/raw/")
        }
    }

    pub fn check_provider_line(url: &str) -> String {
        let arxiv_check = Providers::arxiv(url);
        Providers::github(&arxiv_check)
    }

    pub fn check_provider_domain(url: &str) -> bool {
        let mut valid_domain = false;

        for domain in &Uris::PROVIDERS_DOMAINS {
            if Domains::check(url, domain) {
                valid_domain = true
            }
        }

        valid_domain
    }

    pub async fn scihub(url: &str) -> Result<(String, String), Box<dyn Error>> {
        let paper = Self::extract_doi(url);
        let paper_url = format!("{}{}", Uris::SCIHUB_ADDONS_ENDPOINT, paper);
        let filename = FileRemote::get_filename(&paper_url, true).await?;

        Ok((paper_url, filename))
    }

    pub async fn generic(url: &str) -> Result<(String, String), Box<dyn Error>> {
        let request_uri = url.to_string();
        let filename = FileRemote::get_filename(url, true).await?;

        Ok((request_uri, filename))
    }

    pub async fn get_from_provider(url: &str) -> Result<(String, String), Box<dyn Error>> {
        let filename;
        let request_uri;

        if Domains::check(url, Uris::PROVIDERS_DOMAINS[0]) {
            (request_uri, filename) = Wikipedia::wikipedia(url);
        } else if Domains::check(url, Uris::PROVIDERS_DOMAINS[4]) {
            (request_uri, filename) = Wikipedia::wikisource(url);
        } else if Domains::check(url, Uris::PROVIDERS_DOMAINS[1]) {
            (request_uri, filename) = Self::scihub(url).await?;
        } else {
            (request_uri, filename) = Self::generic(url).await?;
        }

        Ok((request_uri, filename))
    }

}
