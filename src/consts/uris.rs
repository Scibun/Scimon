pub struct Uris;

impl Uris {

    pub const PROVIDERS_DOMAINS: [&'static str; 5] = [
        "wikipedia.org",
        "sci-hub.se",
        "github.com",
        "githubusercontent.com",
        "wikisource.org",
    ];

    // Ravenlib
    pub const RAVENLIB_API_REQUEST: &'static str = "http://localhost/Ravenlib/api/";
    pub const SCIHUB_API_REQUEST: &'static str = "https://addons.scibun.com/scihub?paper=";
    pub const PAIMON_SCRAPE_API_REQUEST: &'static str = "https://addons.scibun.com/scrape?url=";

    // Third-parties
    pub const WIKIPEDIA_API_REQUEST_PDF: &'static str = "https://en.wikipedia.org/api/rest_v1/page/pdf/";
    pub const WIKISOURCE_API_REQUEST_PDF: &'static str = "https://en.wikisource.org/api/rest_v1/page/pdf/";
    
}
