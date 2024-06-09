pub struct Global;

impl Global {

    pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
    pub const APP_AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");
    pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
    pub const APP_HOMEPAGE: &'static str = env!("CARGO_PKG_HOMEPAGE");

    pub const ENV_URL: &'static str = "https://raw.githubusercontent.com/Scibun/Scimon/main/.env.example";
    pub const SETTING_URL: &'static str = "https://raw.githubusercontent.com/Scibun/Scimon/main/scimon.yml";

}
