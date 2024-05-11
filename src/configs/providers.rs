use reqwest;
use scihub_scraper::SciHubScraper;

use std::error::Error;

use crate::{
    addons::scihub::SciHub,
    configs::apis_uri::ApisUri,
    
    utils::{
        url::UrlMisc,
        file::FileMisc,
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

    async fn get_filename(url: &str) -> Result<String, Box<dyn Error>> {
        let filename = FileMisc::detect_name(
            url, reqwest::get(url).await?.headers().get("content-disposition")
        ).await?;

        Ok(filename)
    }

    pub fn arxiv(url: &str) -> String {
        let escape_quotes = UrlMisc::escape_quotes(url);

        if !UrlMisc::check_domain(&escape_quotes, "arxiv.org") {
            escape_quotes.to_owned()
        } else {
            escape_quotes.replace("/abs/", "/pdf/")
        }
    }

    pub fn wikipedia(url: &str) -> (String, String) {
        let wiki_name = UrlMisc::get_last_part(url);
        let wikipedia_region = format!("{}.", UrlMisc::get_subdomain(url));

        let request_url = format!("{}{}", ApisUri::WIKIPEDIA_API_REQUEST_PDF.to_string().replace(
            "en.", &wikipedia_region
        ), wiki_name);

        let filename = format!("{}.pdf", wiki_name);

        (request_url, filename)
    }

    pub async fn scihub(url: &str) -> Result<(String, String), Box<dyn Error>> {
        let mut scraper = SciHubScraper::new();
        let paper = scraper.fetch_paper_pdf_url_by_doi(
            &Self::extract_doi(url)
        ).await?;

        let paper_url = paper.to_string();
        let paper_pdf_url = SciHub::get_pdf_file(&paper_url).await?;
        let filename = Self::get_filename(&paper_url).await?;

        Ok((paper_pdf_url, filename))
    }

    pub async fn generic(url: &str) -> Result<(String, String), Box<dyn Error>> {
        let request_uri = url.to_string();
        let filename = Self::get_filename(url).await?;

        Ok((request_uri, filename))
    }

}
