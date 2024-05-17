extern crate colored;

mod ui;
mod cmd;
mod utils;
mod regex;
mod consts;
mod addons;
mod system;
mod paimon;
mod configs;
mod args_cli;
mod prime_down;

use crate::paimon::Paimon;

#[tokio::main]
async fn main() {
    Paimon::init().await;
}
