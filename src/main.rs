mod cmd;

use std::io;
use std::env;
use is_url::is_url;

use crate::cmd::validation::data::get_first_arg;

use crate::cmd::bootstrap::file_handler::{
    read_local_file,
    read_remote_file
};

#[tokio::main]
async fn main() -> io::Result<()> {
    get_first_arg().unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        "".to_string()
    });

    let args: Vec<String> = env::args().collect();

    let start = &args[1];

    if !is_url(start) {
        if let Err(_) = read_local_file(&start).await {}
    } else {
        if let Err(e) = read_remote_file(start).await {
            eprintln!("Error: {}", e);
        }
    }

    Ok(())
}
