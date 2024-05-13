extern crate url;

use regex::Regex;
use is_url::is_url;
use std::error::Error;

use crate::{
    system::syntax::Macros,
    configs::regex::RegExp,
    utils::remote::FileRemote,
    ui::errors_alerts::ErrorsAlerts,
};

pub struct Reporting;

impl Reporting {
    
    pub async fn check_download_errors(url: &str) -> Result<(), Box<dyn Error>> {
        let final_url = &Macros::remove_macros(url);
        let regex = Regex::new(RegExp::VALIDATE_TAGS).unwrap();

        if regex.is_match(final_url) && !final_url.contains("*") && !final_url.is_empty() {
            let mut url_valid = false;
        
            if !is_url(final_url) {
                let url_invalid = Box::from("Invalid URL provided. Please enter a valid URL");
                ErrorsAlerts::download(url_invalid, final_url);
            } else {
                url_valid = true;
            }

            if FileRemote::get_status_code(final_url).await != 200 && url_valid == true {
                let status_code = Box::from("Failed to retrieve the URL with status code other than 200");
                ErrorsAlerts::download(status_code, final_url);
            }
        }
        
        Ok(())
    }

}
