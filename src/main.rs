extern crate colored;

mod ui;
mod cmd;
mod utils;
mod addons;
mod paimon;
mod configs;
mod args_cli;

use crate::paimon::Paimon;

#[tokio::main]
async fn main() {
    let _ = Paimon::init().await;
}
