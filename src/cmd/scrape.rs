extern crate reqwest;

use std::error::Error;

use serde::Deserialize;

use crate::cmd::download::Download;

use crate::configs::global::Global;

#[derive(Debug, Deserialize)]
struct Item {
    url: String,
    encrypted: bool,
}

#[derive(Debug, Deserialize)]
struct Response {
    list: Vec<Item>,
}

pub struct Scrape;

impl Scrape {

    async fn fetch_items(url: &str) -> Result<Response, Box<dyn std::error::Error>> {
        let url_with_param = Global::PAIMON_SCRAPE_API_REQUEST.to_string() + url;
        let response = reqwest::get(&url_with_param).await?;
        let body = response.bytes().await?;
        
        let data: Response = serde_json::from_slice(&body)?;
        
        Ok(data)
    }

    pub async fn get(scrape: bool, url: &str, no_ignore: bool, no_comments: bool, kindle: Option<String>) -> Result<(), Box<dyn Error>> {
        if scrape {
            match Self::fetch_items(url).await {
                Ok(response) => {
                    for item in response.list {
                        if item.encrypted == false {
                            let _ = Download::run_download_current_line(
                                &item.url, 
                                no_ignore, 
                                no_comments, 
                                kindle.clone()
                            ).await?;
                        }
                    }
                }
                
                Err(e) => eprintln!("Error: {}", e),
            }
        }

        Ok(())
    }

}
