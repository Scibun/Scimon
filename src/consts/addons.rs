pub struct Addons;

impl Addons {

    pub const DOWNLOAD_FILES_URI: &'static str = "https://raw.githubusercontent.com/Scibun/Scimon/main/";
    pub const DEFAULT_CSS_STYLE: &'static str = "https://gist.githubusercontent.com/Kremilly/1e60287dfb89bce16bf160cc06afb2d0/raw/a380edebdc8e0c2598250039adac860b364328bd/github-flavored-md-style.css";
    
    // Plugins
    pub const EXTRACT_COVERS_PLUGIN: &'static str = "https://gist.githubusercontent.com/Kremilly/8ae07310c07093fe4208faf7b2b13b9d/raw/8ba216ea030f009da9557ab8a8b21684776c2c17/extract_covers.py";

    // Scimon
    pub const SCIMON_API_REQUEST: &'static str = "http://localhost/Scimon/api/";
    pub const SCIMON_SCRAPE_API_ENDPOINT: &'static str = "https://addons.scibun.com/scrape?url=";
    pub const SCIMON_URLFILTER_API_ENDPOINT: &'static str = "https://addons.scibun.com/urlfilter?url=";

    pub const README_TEMPLATE_LINK: &'static str = "https://template.scibun.com/";
    
}
