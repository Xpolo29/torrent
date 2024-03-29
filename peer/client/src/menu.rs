use crate::com::{connect, look, receive, seed, send};
use crate::data::{MetaFile, TrackerConfig};
use crate::database::number_to_file_name;
use crate::respons_handler::{ExpectList, ExpectOk, ExpectedAnswer};
use crate::userinput::{display_downloadable_files, get_file_names, get_filename, get_filesize};
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
            1 => search_section(tracker_config.port, &tracker_config.address),
            2 => upload_section(tracker_config.port, &tracker_config.address),
            3 => download_section(tracker_config.port, &tracker_config.address),
            _ => println!("Invalid input, please enter 1, 2 or 3"),
        }
    }
}
fn search_section(tracker_port: u16, tracker_adress: &str) {
    println!("You're in Search");
    let filename = get_filename(io::stdin());
    let op_filesize = get_filesize(io::stdin());
    let look_message = look(filename, op_filesize);
    trace!("Prepared message: {}", look_message);
    if let Some(mut stream) = connect(tracker_port, &tracker_adress.to_string()) {
        send(&mut stream, look_message);
        trace!("Message sent waiting for answer");
        let response = receive(&mut stream);
        trace!("Received {}", response);
        match ExpectList.check_answer(&response) {
            Ok(valeur) => {
                info!("{}", valeur);
            }
            Err(valeur) => {
                error!("{}", valeur);
            }
        }
    }
    let present_files = ExpectList.retrieve_data("".to_string());
    info!("files retrieved {:?}", present_files);
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
    if let Some(mut stream) = connect(tracker_port, &tracker_adress.to_string()) {
        // connect to the tracker
        send(&mut stream, seeded_files); // send the message
        trace!("Message sent waiting for answer");
        let response = receive(&mut stream); // receive the answer
        trace!("Received: {}", response);
        match ExpectOk.check_answer(&response) {
            Ok(valeur) => {
                info!("{}", valeur);
            }
            Err(valeur) => {
                info!("{}", valeur);
            }
        }
        ExpectOk.shutdown(&mut stream);
    }
}
fn download_section(tracker_port: u16, tracker_adress: &str) {
    search_section(tracker_port, tracker_adress);
    // The list of downloadable files should be the result of search section 
    // todo!();
    println!("You're in download");
    // display files along with their size
    // if two files are name the same user should be able to choose which one to download
    display_downloadable_files();
    let choice = get_filename(io::stdin());
    let file_name = number_to_file_name(choice.parse().unwrap());
    println!("You chose to download: {}", file_name);
}
