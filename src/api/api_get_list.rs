extern crate reqwest;

use std::error::Error;
use serde_json::Value;
use std::io::{self, BufRead};
use reqwest::{Client, header};

use crate::configs::global::{
    API_LISTS_ENDPOINT,
    MONLIB_API_REQUEST,
};

use crate::configs::env::env_var;
use crate::utils::misc::remove_initial_character;

use crate::cmd::download::run_download_current_line;

pub async fn api_get_list(list_id: &str, no_ignore: bool, no_comments: bool, kindle: Option<String>) -> Result<String, Box<dyn Error>> {
    let list = remove_initial_character(list_id, '@');
    let mut url = MONLIB_API_REQUEST.to_owned();

    url.push_str(API_LISTS_ENDPOINT);
    url.push_str("/");
    url.push_str(&list);
    url.push_str("/raw");

    let client = Client::builder().danger_accept_invalid_certs(true).build()?;
    let response = client
        .get(&url)
        .header(header::AUTHORIZATION, format!("Bearer {}", env_var("MONLIB_API_KEY")))
        .send().await?;

    if response.status().is_success() {
        let result = String::new();
        let mut is_json = true;

        let data = response.text().await?;

        if let Ok(json_data) = serde_json::from_str::<Value>(&data) {
            if let Some(message) = json_data.get("message") {
                if let Some(message_str) = message.as_str() {
                    return Ok(message_str.to_string());
                }
            }
        } else {
            is_json = false;
        }

        if !is_json {
            let lines_iter = io::Cursor::new(&data).lines();

            for line_result in lines_iter {
                let line = line_result?;
                run_download_current_line(&line, no_ignore, no_comments, kindle.clone()).await?;
            }
        }

        Ok(result)
    } else {
        Err(
            format!(
                "Error while making the request: {:?}", response.status()
            ).into()
        )
    }
}
