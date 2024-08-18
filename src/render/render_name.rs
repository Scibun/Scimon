use crate::{
    consts::uris::Uris,
    utils::domains::Domains,
};

pub struct RenderName {
    pub domain: String,
    pub url_slices: Vec<String>,
}

impl RenderName {

    pub fn new(url: &str) -> Self {
        Self {
            domain: Domains::get(url),
            url_slices: url.split("/").map(|s| s.to_string()).collect(),
        }
    }

    fn github_repo_name(&self) -> String {
        let segments = &self.url_slices;
        let repo_name = &segments[segments.len() - 3];
        repo_name.to_string()
    }

    fn gitlab_repo_name(&self) -> String {
        let segments = &self.url_slices;
        let repo_name = &segments[segments.len() - 5];
        repo_name.to_string()
    }

    fn bitbucket_repo_name(&self) -> String {
        let segments = &self.url_slices;
        let repo_name = &segments[segments.len() - 4];
        repo_name.to_string()
    }

    fn codeberg_repo_name(&self) -> String {
        let segments = &self.url_slices;
        let repo_name = &segments[segments.len() - 5];
        repo_name.to_string()
    }

    fn generic(&self) -> String {
        if let Some(last_part) = self.url_slices.last() {
            last_part.to_string()
        } else {
            String::new()
        }
    }

    pub fn get_final_name(&self) -> String {
        match &self.domain {
            domain if domain == Uris::PROVIDERS_DOMAINS[2] => self.github_repo_name(),
            domain if domain == Uris::PROVIDERS_DOMAINS[3] => self.gitlab_repo_name(),
            domain if domain == Uris::PROVIDERS_DOMAINS[4] => self.bitbucket_repo_name(),
            domain if domain == Uris::PROVIDERS_DOMAINS[5] => self.codeberg_repo_name(),
            _ => self.generic(),
        }
    }

}
