use crate::{
    consts::uris::Uris,
    
    utils::{
        url::UrlMisc,
        domains::Domains,
    },
};

pub struct FileName;

impl FileName {

    fn github_repo_name(url: &str) -> String {
        let segments: Vec<&str> = url.split('/').collect();
        let repo_name = segments[segments.len() - 3];
        repo_name.to_string()
    }

    pub fn get_final_name(url: &str) -> String {
        let domain = Domains::get(url);

        match domain.as_str() {
            domain if domain == Uris::PROVIDERS_DOMAINS[2] => Self::github_repo_name(url),
            _ => UrlMisc::get_last_part(url),
        }
    }

}