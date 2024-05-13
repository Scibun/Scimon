use std::error::Error;
use scihub_scraper::SciHubScraper;

use crate::{
    addons::scihub::SciHub,
    
    utils::{
        url::UrlMisc,
        remote::FileRemote,
    },
};

pub struct Providers;

impl Providers {

    const WIKIPEDIA_API_REQUEST_PDF: &'static str = "https://en.wikipedia.org/api/rest_v1/page/pdf/";

    const PROVIDERS_DOMAINS: [&'static str; 4] = [
        "wikipedia.org",
        "sci-hub.se",
        "github.com",
        "githubusercontent.com"
    ];

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

        if !UrlMisc::check_domain(&escape_quotes, Self::PROVIDERS_DOMAINS[1]) {
            escape_quotes.to_owned()
        } else {
            escape_quotes.replace("/abs/", "/pdf/")
        }
    }
    
    pub fn github(url: &str) -> String {
        let escape_quotes = UrlMisc::escape_quotes(url);

        if !UrlMisc::check_domain(&escape_quotes, Self::PROVIDERS_DOMAINS[2]) {
            escape_quotes.to_owned()
        } else if !UrlMisc::check_domain(&escape_quotes, Self::PROVIDERS_DOMAINS[3]) {
            escape_quotes.to_owned()
        } else {
            escape_quotes.replace("/blob/", "/raw/")
        }
    }

    pub fn wikipedia(url: &str) -> (String, String) {
        let wiki_name = UrlMisc::get_last_part(url);
        let wikipedia_region = format!("{}.", UrlMisc::get_subdomain(url));

        let request_url = format!("{}{}", Self::WIKIPEDIA_API_REQUEST_PDF.to_string().replace(
            "en.", &wikipedia_region
        ), wiki_name);

        let filename = format!("{}.pdf", wiki_name);

        (request_url, filename)
    }

    pub fn check_provider_domain(url: &str) -> bool {
        let mut valid_domain = false;

        for domain in &Self::PROVIDERS_DOMAINS {
            if UrlMisc::check_domain(url, domain) {
                valid_domain = true
            }
        }

        valid_domain
    }

    pub async fn scihub(url: &str) -> Result<(String, String), Box<dyn Error>> {
        let mut scraper = SciHubScraper::new();

        let paper = scraper.fetch_paper_pdf_url_by_doi(
            &Self::extract_doi(url)
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

        if UrlMisc::check_domain(url, Self::PROVIDERS_DOMAINS[0]) {
            (request_uri, filename) = Self::wikipedia(url);
        } else if UrlMisc::check_domain(url, Self::PROVIDERS_DOMAINS[1]) {
            (request_uri, filename) = Self::scihub(url).await?;
        } else {
            (request_uri, filename) = Self::generic(url).await?;
        }

        Ok((request_uri, filename))
    }

}
