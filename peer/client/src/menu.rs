use std::io;
use crate::userinput::get_file_names;
use crate::com::{seed, send};
use crate::data::{MetaFile,TrackerConfig};

pub fn display_menu(tracker_config: TrackerConfig) {
    loop {

        println!("Main Menu");
        println!("1. Search");
        println!("2. Upload");
        println!("3. Download");
        
        let mut input = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
        
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        
        match input {
            // Escape should get back to menu from search, upload and download
            1 => search_section(),
            2 => upload_section(tracker_config.port,&tracker_config.address),
            3 => download_section(),
            _ => println!("Invalid input, please enter 1, 2 or 3"),
        }
        
    }
    }
fn search_section() {
        println!("You're in Search")
}
fn upload_section(tracker_port:u16, tracker_adress:&str) {
    println!("You're in upload");
    let seeded_files = get_file_names(io::stdin());
    let seeded_files: Vec<MetaFile> = seeded_files.
    into_iter().
    map(|file| MetaFile::new(file.to_string())).
    collect();
    let seeded_files = seed(seeded_files, "8080".to_string(), "".to_string());
    send(seeded_files,tracker_port,tracker_adress.to_string());
}
fn download_section() {
    println!("You're in download")
}
