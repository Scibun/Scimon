pub struct Uris;

impl Uris {

    pub const PROVIDERS_DOMAINS: [&'static str; 2] = [
        "wikipedia.org",
        "wikisource.org",
    ];

    // Scimon
    pub const SCIMON_API_REQUEST: &'static str = "http://localhost/Scimon/api/";
    pub const SCIMON_SCRAPE_API_ENPOINT: &'static str = "https://addons.scibun.com/scrape?url=";

    pub const README_TEMPLATE_LINK: &'static str = "https://template.scibun.com/";

    // Third-parties
    pub const WIKIPEDIA_API_REQUEST_PDF: &'static str = "https://en.wikipedia.org/api/rest_v1/page/pdf/";
    pub const WIKISOURCE_API_REQUEST_PDF: &'static str = "https://en.wikisource.org/api/rest_v1/page/pdf/";
    
}
