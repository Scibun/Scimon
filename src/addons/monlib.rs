extern crate reqwest;
extern crate colored;

use webbrowser;
use colored::*;

use std::{
    fmt,
    error::Error,
    io::{self, BufRead}
};

use serde_json::Value;
use serde::Deserialize;

use reqwest::{
    Client,
    header,
    multipart
};

use crate::{
    configs::{
        env::Env,
        apis_uri::ApisUri,
    }, utils::{
        misc::Misc,
        file::FileMisc,
    }, cmd::{
        syntax::Lexico,
        download::Download,
    }
};

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
}

impl fmt::Display for ApiError {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::Message(msg) => write!(f, "{}", msg),
        }
    }
    
}

impl Error for ApiError {}

pub struct Monlib;

impl Monlib {

    async fn create_list(file_path: &str, title: &str, privacy: Option<&str>) -> Result<String, Box<dyn Error>> {
        let mut url = ApisUri::MONLIB_API_REQUEST.to_owned();
        url.push_str(ApisUri::API_LISTS_ENDPOINT);
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
                    FileMisc::get_file_name_string(file_path)
                )
            )
            .text("title", title.to_owned())
            .text("privacy", privacy_string);
    
        let response = client
            .post(&url)
            .header(
                header::AUTHORIZATION, format!(
                    "Bearer {}", Env::env_var("MONLIB_API_KEY")
                )
            )
            .multipart(form_data)
            .send()
            .await?;
    
        if response.status().is_success() {
            let response_text = response.text().await?;
            let result: ApiSuccessResponse = serde_json::from_str(&response_text)?;
            let message = ApiError::Message(result.data.message);
    
            println!("[{}] {}", Misc::date_time().blue(), message.to_string().green());
            webbrowser::open(&result.data.url)?;
    
            Ok(
                response_text.to_string()
            )
        } else {
            let response_text = response.text().await?;
    
            if let Ok(error_response) = serde_json::from_str::<ErrorResponse>(&response_text) {
                let message = ApiError::Message(error_response.message);
                println!("[{}] {}", Misc::date_time().blue(), message.to_string().red());
    
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

    pub async fn publish(publish: bool, file: Option<String>, title: Option<String>, privacy: Option<String>) {
        if publish {
            if let (Some(file_path), Some(title)) = (file, title) {
                let privacy = privacy;
    
                let _ = Self::create_list(
                    &file_path, &title, privacy.as_deref()
                ).await;
            } else {
                eprintln!("Error: {}", "Both 'file' and 'title' are required for publishing a library.".red());
            }
        }
    }

    pub async fn get(list_id: &str, no_ignore: bool, no_comments: bool) -> Result<String, Box<dyn Error>> {
        let list = Misc::remove_initial_character(list_id, '@');
        let mut url = ApisUri::MONLIB_API_REQUEST.to_owned();
    
        url.push_str(ApisUri::API_LISTS_ENDPOINT);
        url.push_str("/");
        url.push_str(&list);
        url.push_str("/raw");
    
        let client = Client::builder().danger_accept_invalid_certs(true).build()?;
        let response = client
            .get(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", Env::env_var("MONLIB_API_KEY")))
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
                    let path = Lexico::handle_get_path(&line);
                    let _ = FileMisc::new_path(&path);

                    Download::download_file(
                        &line, 
                        &path,
                        no_ignore, 
                        no_comments
                    ).await?;
                }
            }
    
            Ok(result)
        } else {
            let response_text = response.text().await?;
    
            if let Ok(error_response) = serde_json::from_str::<ErrorResponse>(&response_text) {
                let message = ApiError::Message(error_response.message);
                println!("[{}] {}", Misc::date_time().blue(), message.to_string().red());
    
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
    
}
