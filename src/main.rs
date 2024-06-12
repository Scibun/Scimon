mod ui;
mod cmd;
mod utils;
mod regexp;
mod consts;
mod addons;
mod system;
mod monset;
mod scimon;
mod configs;
mod args_cli;
mod prime_down;

use crate::scimon::Scimon;

#[tokio::main]
async fn main() {
    Scimon::init().await;
}
