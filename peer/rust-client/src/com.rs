// src/com.rs

use std::io::Write;
use std::net::TcpStream;

use crate::config::FileProp;
// Send port and available files to tracker
pub fn send_port_seed_to_tracker(port: u16, files: Vec<FileProp>) {
    // Connect to the tracker unwrap function takes the Ok variant of the Result and returns the value inside
    // expect() si le résultat est Err, le programme crash avec le message passé en paramètre
    let mut stream = TcpStream::connect("127.0.0.1:7878").expect("Pas réussis à se connecter au tracker");
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
        "< announce listen {} seed [{}]",
        port,
        files_string.join(" ")
    );
    stream.write(msg.as_bytes()).unwrap();
}