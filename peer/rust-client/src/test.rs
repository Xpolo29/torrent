use std::fs;
use std::env::args;

fn main() {
    // Get arguments from command line
    let mut arguments = args();
    arguments.next(); // Skip the program name

    // Check if there's a file path argument
    let filename = match arguments.next() {
        Some(path) => path,
        None => {
            eprintln!("Usage: {} <filepath>", arguments.next().unwrap());
            return;
        }
    };

    // Get file size
    let size_result = fs::metadata(&filename);

    // Handle potential errors
    match size_result {
        Ok(meta) => println!("File size: {} bytes", meta.len()),
        Err(err) => eprintln!("Error: {}", err),
    }
}