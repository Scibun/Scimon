use crate::{
    consts::uris::Uris,
    
    utils::{
        url::UrlMisc,
        domains::Domains,
    },
};

pub struct FileName;

impl FileName {

    fn split_url(url: &str) -> Vec<&str> {
        url.split('/').collect()
    }

    fn github_repo_name(url: &str) -> String {
        let segments = Self::split_url(url);
        let repo_name = segments[segments.len() - 3];
        repo_name.to_string()
    }

    fn gitlab_repo_name(url: &str) -> String {
        let segments = Self::split_url(url);
        let repo_name = segments[segments.len() - 5];
        repo_name.to_string()
    }

    pub fn get_final_name(url: &str) -> String {
        let domain = Domains::get(url);

        match domain.as_str() {
            domain if domain == Uris::PROVIDERS_DOMAINS[2] => Self::github_repo_name(url),
            domain if domain == Uris::PROVIDERS_DOMAINS[3] => Self::gitlab_repo_name(url),
            _ => UrlMisc::get_last_part(url),
        }
    }

}