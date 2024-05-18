mod ui;
mod cmd;
mod utils;
mod regexp;
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
