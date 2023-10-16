mod cmd;

use std::io;
use std::env;
use is_url::is_url;

use crate::cmd::validation::data::get_first_arg;

use crate::cmd::bootstrap::file_handler::{
    read_paimon_local_file,
    read_paimon_remote_file
};

#[tokio::main]
async fn main() -> io::Result<()> {
    get_first_arg().unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        "".to_string()
    });

    let args: Vec<String> = env::args().collect();

    let start = &args[1];

    let second_arg = match args.get(2) {
        Some(arg) => arg,
        None => ""
    };

    if !is_url(start) {
        if let Err(_) = read_paimon_local_file(start, second_arg).await {}
    } else {
        if let Err(e) = read_paimon_remote_file(start, second_arg).await {
            eprintln!("Error: {}", e);
        }
    }

    Ok(())
}
