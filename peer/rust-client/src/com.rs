// src/com.rs
use crate::network_config::{ExpectList, ExpectOk, ExpectedAnswer, FileProp, TrackerConfig}; // Import trait from config
use crate::parse::goes_well;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

fn send_message(msg: String, expected_answer: &dyn ExpectedAnswer) {
    let tracker_config = TrackerConfig::from_config().unwrap();
    let ip = tracker_config.ip;
    let port = tracker_config.port;
    println!("Connecting to {}:{}", ip, port);
    let mut stream = TcpStream::connect(format!("{}:{}", ip, port)).expect("Could not connect to tracker");
    stream.write(msg.as_bytes()).unwrap();
    let mut reader = BufReader::new(stream);
    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap();
    match goes_well(buffer, expected_answer) {
        Ok(response) => {
            println!("Réponse: {}", response);
        }
        Err(e) => {
            println!("Erreur: {}", e);
        }
    }
}
// Send port and available files to tracker
pub fn send_port_seed_to_tracker(port: u16, files: Vec<FileProp>) {
    /*
    into_iter() : transform the vector into an iterator
    map() : apply a function to each element of the iterator
    collect() : transform the iterator into a vector
    join() : concatenate the elements of the vector into a single string
     */
    let files_string: Vec<String> = files
        .into_iter()
        .map(|file| {
            format!(
                "{} {} {} {}",
                file.file_name, file.length, file.piece_size, file.hash
            )
        })
        .collect();

    let msg = format!(
        "announce listen {} seed [{}]\r\n",
        port,
        files_string.join(" ")
    );
    send_message(msg, &ExpectOk);
}

// works if there is one criterion
pub fn send_search_to_tracker(criterions: String) {
    if criterions.is_empty() {
        send_message("look []\r\n".to_string(), &ExpectList);
    } else {
        send_message(
            format!("look [filename=\"{}\"]\r\n", criterions),
            &ExpectList,
        );
    }
}

pub fn send_wanted_files_to_tracker(files: Vec<String>) {
    let msg = format!("getfile [{}]\r\n", files.join(" "));
    send_message(msg, &ExpectList);
}

#[cfg(test)]
mod tests {}
