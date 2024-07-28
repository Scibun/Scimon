mod ui;
mod cmd;
mod ext;
mod utils;
mod regexp;
mod consts;
mod addons;
mod system;
mod syntax;
mod scimon;
mod render;
mod configs;
mod args_cli;
mod generator;

use crate::scimon::Scimon;

#[tokio::main]
async fn main() {
    Scimon::init().await;
}
