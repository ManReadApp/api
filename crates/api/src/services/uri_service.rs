use manread_scraper::ExternalSite;

pub struct UriService {
    sites: Vec<ExternalSite>,
}

impl UriService {
    pub fn new(sites: Vec<ExternalSite>) -> Self {
        Self { sites }
    }
    pub fn get_uri(&self, url: &str) -> String {
        for site in &self.sites {
            if site.check(url) {
                return site.uri.clone();
            }
        }
        "unknown".to_string()
    }
}
