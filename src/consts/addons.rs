pub struct Addons;

impl Addons {

    pub const DOWNLOAD_FILES_URI: &'static str = "https://raw.githubusercontent.com/Scibun/Scimon/main/";
    pub const DEFAULT_CSS_STYLE: &'static str = "https://addons.scibun.com/static/md-default.css";
    
    // Plugins
    pub const EXTRACT_COVERS_PLUGIN: &'static str = "https://raw.githubusercontent.com/Scibun/MonPlugins/main/extract_covers.py";
    pub const INSTALL_REQUIREMENTS_PLUGINS: &'static str = "https://raw.githubusercontent.com/Scibun/MonPlugins/main/requirements.txt";

    // Scimon
    pub const SCIMON_API_REQUEST: &'static str = "http://localhost/Scimon/api/";
    pub const SCIMON_SCRAPE_API_ENDPOINT: &'static str = "https://addons.scibun.com/scrape?url=";
    pub const SCIMON_URLFILTER_API_ENDPOINT: &'static str = "https://addons.scibun.com/urlfilter?url=";

    pub const README_TEMPLATE_LINK: &'static str = "https://template.scibun.com/";
    
}
