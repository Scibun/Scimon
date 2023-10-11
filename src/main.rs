mod download;

use std::env;
use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead, BufReader};

#[tokio::main]
async fn main() -> io::Result<()> {
    // Get the command line arguments.
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please pass the file name as an argument.");
        return Ok(());
    }

    let file = &args[1];

    // Check if file exists
    if !Path::new(file).exists() {
        println!("The file {} not exists.", file);
        return Ok(());
    }

    // Check if the file has a .txt extension.
    if !file.ends_with(".txt") {
        println!("The file {} is not a .txt", file);
        return Ok(());
    }

    // Open and read the file line by line.
    let file = File::open(file)?;
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(l) => l,

            Err(e) => {
                eprintln!("Error reading line: {}", e);
                continue; // jump to the next line
            }
        };

        let result = download::pdf::download_and_detect_name(&line).await;
        
        match result {
            Ok(name_string) => {
                println!("Downloaded file name: {}", name_string);
            },
            Err(e) => {
                eprintln!("Error downloading or detecting the name: {}", e);
            }
        }
    }

    Ok(())
}
