// src/com.rs
use std::io::{Read, Write};
use std::net::TcpStream;
use crate::config::{FileProp, Message}; // Import enum from config
use crate::parse::parse_answer;


// Wait for the answer of the tracker
fn goes_well(buffer: String, message: Message) {
    match message {
        Message::OK => {
            parse_answer(buffer, Message::OK);
        }
        Message::LIST => {
            parse_answer(buffer, Message::LIST);
        }
    }
}

fn send_message(msg: String, answer_type: Message) {
    // Connect to the tracker unwrap function takes the Ok variant of the Result and returns the value inside
    // expect() si le résultat est Err, le programme crash avec le message passé en paramètre
    let ip ="127.0.1";
    let port = "7878";
    // Connect to the tracker
    let mut stream = TcpStream::connect(format!("{}:{}", ip, port)).unwrap();
    // Send the message
    stream.write(msg.as_bytes()).unwrap();
    // Listen for the answer
    let mut buffer = String::new();
    stream.read_to_string(&mut buffer).unwrap();
    // Handle awaited answer
    goes_well(buffer, answer_type);
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
    send_message(msg, Message::OK);
}

// works if there is one criterion
pub fn send_search_to_tracker(criterions: String) {
    if criterions.is_empty() {
        send_message("look\r\n".to_string(), Message::OK);
    } else {
        send_message(format!("look [filename=\"{}\"]\r\n", criterions), Message::OK);
    }
}
