pub struct Global;

impl Global {

    pub const APP_NAME: &'static str = "Paimon";
    pub const APP_VERSION: &'static str = "0.0.1";
    pub const APP_AUTHOR: &'static str = "@Ravenlib";
    pub const APP_HOMEPAGE: &'static str = "https://github.com/Ravenlib/Paimon";

    pub const ENV_URL: &'static str = "https://pastebin.com/raw/wZGaNtsL";

    pub const PB_STYLE: &'static str = "[{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})";

}
