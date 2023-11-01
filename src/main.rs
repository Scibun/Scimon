mod cmd;

extern crate colored;
extern crate figlet_rs;

use std::io;
use clap::Parser;

use colored::*;
use figlet_rs::FIGfont;

use crate::cmd::configs::global::{
    APP_NAME,
    APP_AUTHOR,
    APP_VERSION,
    APP_HOMEPAGE
};

use crate::cmd::env::configs::download_env_file;
use crate::cmd::bootstrap::file_handler::read_paimon_file;

#[derive(Parser)]
struct Args{
    #[arg(short, long)]
    run: String,

    #[arg(long)]
    noignore: bool,

    #[arg(long)]
    kindle: Option<String>,

    #[arg(long)]
    inspect: bool,

    #[arg(short, long)]
    publish: bool,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    if let Err(err) = download_env_file().await {
        eprintln!("Error: {}", err);
    }

    let args_parser : Args = Args::parse();

    let run = args_parser.run;
    let standard_font = FIGfont::standard().unwrap();
    
    if let Some(title) = standard_font.convert(APP_NAME) {
        println!("{}", title.to_string().blue());
        println!("-------------------------------------------------------------------");
        println!("📜 Version: {}", APP_VERSION.yellow());
        println!("🏠 Homepage: {} | {}", APP_HOMEPAGE.blue(), APP_AUTHOR.green());
        println!("-------------------------------------------------------------------");
    }

    read_paimon_file(&run, args_parser.noignore, args_parser.kindle).await;

    Ok(())
}
