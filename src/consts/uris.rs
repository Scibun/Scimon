pub struct Uris;

impl Uris {

    pub const PROVIDERS_DOMAINS: [&'static str; 2] = [
        "wikipedia.org",
        "wikisource.org",
    ];

    pub const WIKIPEDIA_API_REQUEST_PDF: &'static str = "https://en.wikipedia.org/api/rest_v1/page/pdf/";
    pub const WIKISOURCE_API_REQUEST_PDF: &'static str = "https://en.wikisource.org/api/rest_v1/page/pdf/";
    
}
