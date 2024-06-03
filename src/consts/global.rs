pub struct Global;

impl Global {

    pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
    pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
    pub const APP_AUTHOR: &'static str = "@Kremilly";
    pub const APP_HOMEPAGE: &'static str = "https://github.com/Kremilly/Paimon";

    pub const ENV_URL: &'static str = "https://pastebin.com/raw/wZGaNtsL";
    pub const SETTING_URL: &'static str = "https://pastebin.com/raw/jVdvKhWg";

    pub const PB_STYLE: &'static str = "[{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})";

}
