use std::error::Error;
use scihub_scraper::SciHubScraper;

use crate::{
    consts::uris::Uris,

    addons::{
        scihub::SciHub,
        wikipedia::Wikipedia,
    },
    
    utils::{
        url::UrlMisc,
        remote::FileRemote,
    },
};

pub struct Providers;

impl Providers {

    pub fn arxiv(url: &str) -> String {
        let escape_quotes = UrlMisc::escape_quotes(url);

        if !UrlMisc::check_domain(&escape_quotes, Uris::PROVIDERS_DOMAINS[1]) {
            escape_quotes.to_owned()
        } else {
            escape_quotes.replace("/abs/", "/pdf/")
        }
    }
    
    pub fn github(url: &str) -> String {
        let escape_quotes = UrlMisc::escape_quotes(url);

        if !UrlMisc::check_domain(&escape_quotes, Uris::PROVIDERS_DOMAINS[2]) {
            escape_quotes.to_owned()
        } else if !UrlMisc::check_domain(&escape_quotes, Uris::PROVIDERS_DOMAINS[3]) {
            escape_quotes.to_owned()
        } else {
            escape_quotes.replace("/blob/", "/raw/")
        }
    }

    pub fn check_provider_domain(url: &str) -> bool {
        let mut valid_domain = false;

        for domain in &Uris::PROVIDERS_DOMAINS {
            if UrlMisc::check_domain(url, domain) {
                valid_domain = true
            }
        }

        valid_domain
    }

    pub async fn scihub(url: &str) -> Result<(String, String), Box<dyn Error>> {
        let mut scraper = SciHubScraper::new();

        let paper = scraper.fetch_paper_pdf_url_by_doi(
            &SciHub::extract_doi(url)
        ).await?;

        let paper_url = paper.to_string();
        let paper_pdf_url = SciHub::get_pdf_file(&paper_url).await?;
        let filename = FileRemote::get_filename(&paper_url).await?;

        Ok((paper_pdf_url, filename))
    }

    pub async fn generic(url: &str) -> Result<(String, String), Box<dyn Error>> {
        let request_uri = url.to_string();
        let filename = FileRemote::get_filename(url).await?;

        Ok((request_uri, filename))
    }

    pub async fn get_from_provider(url: &str) -> Result<(String, String), Box<dyn Error>> {
        let filename;
        let request_uri;

        if UrlMisc::check_domain(url, Uris::PROVIDERS_DOMAINS[0]) {
            (request_uri, filename) = Wikipedia::wikipedia(url);
        } else if UrlMisc::check_domain(url, Uris::PROVIDERS_DOMAINS[4]) {
            (request_uri, filename) = Wikipedia::wikisource(url);
        } else if UrlMisc::check_domain(url, Uris::PROVIDERS_DOMAINS[1]) {
            (request_uri, filename) = Self::scihub(url).await?;
        } else {
            (request_uri, filename) = Self::generic(url).await?;
        }

        Ok((request_uri, filename))
    }

}
