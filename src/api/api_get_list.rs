extern crate reqwest;
extern crate colored;

use colored::*;

use std::fmt;
use std::error::Error;
use serde_json::Value;
use serde::Deserialize;
use std::io::{self, BufRead};
use reqwest::{Client, header};

use crate::configs::global::{
    API_LISTS_ENDPOINT,
    MONLIB_API_REQUEST,
};

use crate::configs::env::env_var;
use crate::utils::misc::date_time;
use crate::utils::misc::remove_initial_character;
use crate::cmd::download::run_download_current_line;

#[derive(Debug, Deserialize)]
struct ErrorResponse {
    message: String,
}

#[derive(Debug)]
enum ApiError {
    Message(String),
    Response(ErrorResponse),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::Message(msg) => write!(f, "{}", msg),
            ApiError::Response(response) => write!(f, "{}", response.message),
        }
    }
}

impl Error for ApiError {}

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
        let response_text = response.text().await?;

        if let Ok(error_response) = serde_json::from_str::<ErrorResponse>(&response_text) {
            let message = ApiError::Message(error_response.message);
            println!("[{}] {}", date_time().blue(), message.to_string().red());

            Ok(message.to_string())
        } else {
            Err(
                ApiError::Message(
                    format!("Error: internal server error")
                ).into()
            )
        }
    }
}
