extern crate reqwest;
extern crate colored;

use std::fmt;
use webbrowser;
use colored::*;
use std::error::Error;
use serde::Deserialize;
use reqwest::{Client, header, multipart};

use crate::configs::global::{
    API_LISTS_ENDPOINT,
    MONLIB_API_REQUEST,
};

use crate::configs::env::env_var;
use crate::utils::misc::date_time;
use crate::utils::file::get_file_name_string;

#[derive(Debug, Deserialize)]
struct Data {
    message: String,
    url: String,
}

#[derive(Debug, Deserialize)]
struct ApiSuccessResponse {
    data: Data,
}

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

pub async fn api_publish_list(file_path: &str, title: &str, privacy: Option<&str>) -> Result<String, Box<dyn Error>> {
    let mut url = MONLIB_API_REQUEST.to_owned();
    url.push_str(API_LISTS_ENDPOINT);
    url.push_str("/create");

    let client = Client::builder().danger_accept_invalid_certs(true).build()?;

    let file_content = tokio::fs::read(file_path).await?;
    let privacy_string: String = privacy.map_or_else(|| String::default(), |s| s.to_string());

    let form_data = multipart::Form::new()
        .part(
            "file", reqwest::multipart::Part::stream(
                file_content
            )
            .file_name(
                get_file_name_string(file_path)
            )
        )
        .text("title", title.to_owned())
        .text("privacy", privacy_string);

    let response = client
        .post(&url)
        .header(
            header::AUTHORIZATION, format!(
                "Bearer {}", env_var("MONLIB_API_KEY")
            )
        )
        .multipart(form_data)
        .send()
        .await?;

    if response.status().is_success() {
        let response_text = response.text().await?;
        let result: ApiSuccessResponse = serde_json::from_str(&response_text)?;
        let message = ApiError::Message(result.data.message);

        println!("[{}] {}", date_time().blue(), message.to_string().green());
        webbrowser::open(&result.data.url)?;

        Ok(
            response_text.to_string()
        )
    } else {
        let response_text = response.text().await?;

        if let Ok(error_response) = serde_json::from_str::<ErrorResponse>(&response_text) {
            let message = ApiError::Message(error_response.message);
            println!("[{}] {}", date_time().blue(), message.to_string().red());

            Ok(
                message.to_string()
            )
        } else {
            Err(
                ApiError::Message(
                    format!("Error: internal server error")
                ).into()
            )
        }
    }
}
