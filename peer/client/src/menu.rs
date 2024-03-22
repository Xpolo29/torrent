use crate::com::{receive, seed, send, Answer, ExpectOk, ExpectedAnswer, connect};
use crate::data::{MetaFile, TrackerConfig};
use crate::userinput::get_file_names;
use log::{error, info, trace};
use std::io;
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
            Err(_) => continue,
        };

        match input {
            // Escape should get back to menu from search, upload and download
            1 => search_section(),
            2 => upload_section(tracker_config.port, &tracker_config.address),
            3 => download_section(),
            _ => println!("Invalid input, please enter 1, 2 or 3"),
        }
    }
}
fn search_section() {
    println!("You're in Search")
}
fn upload_section(tracker_port: u16, tracker_adress: &str) {
    println!("You're in upload");
    let seeded_files = get_file_names(io::stdin()); // take the files the user wish to seed
    let seeded_files: Vec<MetaFile> = seeded_files
        .into_iter()
        .map(|file| MetaFile::new(file.to_string()))
        .collect(); // Create vector of Metafiles out of the files name
    let seeded_files = seed(seeded_files, "8080".to_string(), "".to_string()); // create the message
    trace!("Prepared message: {}", seeded_files);
    let mut stream = connect(tracker_port, &tracker_adress.to_string()); // connect to the tracker
    let mut stream = match stream {
        Ok(stream) => stream,
        Err(e) => {
            error!("Could not connect to tracker: {}", e);
            return;
        }
    };
    send(&mut stream, seeded_files); // send the message
    trace!("Message sent waiting for answer");
    let response = receive(&ExpectOk, stream); // receive the answer
    trace!("Received: {}", response);
    match ExpectOk.check_answer(&response) {
        Ok(valeur) => {
            info!("{}", valeur);
        }
        Err(valeur) => {
            info!("{}", valeur);
        }
    }
}
fn download_section() {
    println!("You're in download")
}
