use std::io;
use crate::userinput::get_file_names;
use crate::com::seed;
use crate::data::MetaFile;

pub fn display_menu() {
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
            2 => upload_section(),
            3 => download_section(),
            _ => println!("Invalid input, please enter 1, 2 or 3"),
        }
        
    }
    }
fn search_section() {
        println!("You're in Search")
}
fn upload_section() {
    println!("You're in upload");
    let seeded_files = get_file_names(io::stdin());
    let seeded_files: Vec<MetaFile> = seeded_files.
    into_iter().
    map(|file| MetaFile::new(file.to_string())).
    collect();
    let seeded_files = seed(seeded_files, "8080".to_string(), "".to_string());
    println!("{}", seeded_files);

}
fn download_section() {
    println!("You're in download")
}
