pub struct Global;

impl Global {

    pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
    pub const APP_AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");
    pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
    pub const APP_HOMEPAGE: &'static str = env!("CARGO_PKG_HOMEPAGE");

    pub const DOWNLOAD_FILES_URI: &'static str = "https://raw.githubusercontent.com/Scibun/Scimon/main/";
    pub const DEFAULT_CSS_STYLE: &'static str = "https://gist.githubusercontent.com/mattdanielbrown/9e357236247281088bfc76349867604f/raw/93f30e10eec9abe04673d536138c1ed0fc0cdc81/GitHub-flavored-markdown-styles.css";

}
