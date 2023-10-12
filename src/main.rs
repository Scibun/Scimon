mod cmd;  // Ensure the cmd folder exists and contains the validation and download modules.

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

// Imports of the functions.
use crate::cmd::download::download::{download_and_detect_name};

use crate::cmd::validation::validation::{
    validate_url, 
    get_first_arg, 
    validate_file 
};

#[tokio::main]
async fn main() -> io::Result<()> {
    // Check the args passed via cli
    get_first_arg().unwrap_or_else(|e| {
        eprintln!("Error: {}", e); // Return the error message
        "".to_string() // Return a empty string or a default value.
    });

    // Get the command line arguments.
    let args: Vec<String> = env::args().collect();

    // Get first arg
    let file = &args[1];

    // Validate file
    if let Err(e) = validate_file(file) {
        eprintln!("{}", e); // Return the error message
        return Ok(());
    }

    // Open and read the file line by line.
    let file = File::open(file)?;
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(l) => l,

            // Print the error message
            Err(e) => {
                eprintln!("Error: reading line: {}", e); // Return the error message
                continue; // jump to the next line
            }
        };

        // Check if current line is a url
        if let Err(e) = validate_url(&line) {
            eprintln!("{}", e); // Return the error message
            return Ok(());
        }

        // Run download file
        let result = download_and_detect_name(&line).await;
        
        match result {
            Ok(file_name) => {
                println!("Downloaded file name: {}", file_name); // Show the success message
            },

            Err(e) => {
                eprintln!("Error downloading or detecting the name: {}", e); // Return the error message
            }
        }
    }

    Ok(())
}
