mod ui;
mod cmd;
mod utils;
mod regexp;
mod consts;
mod addons;
mod system;
mod monset;
mod scibun;
mod configs;
mod args_cli;
mod prime_down;

use crate::scibun::Scibun;

#[tokio::main]
async fn main() {
    Scibun::init().await;
}
