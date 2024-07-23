mod ui;
mod cmd;
mod utils;
mod regexp;
mod consts;
mod addons;
mod system;
mod syntax;
mod scimon;
mod configs;
mod args_cli;
mod generator;
mod render;

use crate::scimon::Scimon;

#[tokio::main]
async fn main() {
    Scimon::init().await;
}
