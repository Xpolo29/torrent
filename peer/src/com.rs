//! communication between the peer and the tracker
use crate::data::MetaFile;
use log::{debug, error, info};
use std::io::{BufReader, Read, Write};
use std::net::TcpStream;
use crate::db::{get_seeding_files, get_leeching_files};
/// # Examples
///
/// ```
/// let seeded_files = vec![
///     MetaFile {
///         file_name: "file1.txt".to_string(),
///         length: 100,
///         piece_size: 10,
///         hash: "abc123".to_string(),
///     },
///     MetaFile {
///         file_name: "file2.txt".to_string(),
///         length: 200,
///         piece_size: 20,
///         hash: "def456".to_string(),
///     },
/// ];
/// let peer_port = "8000".to_string();
/// let leeched_files = "file3.txt".to_string();
/// let message = seed(seeded_files, peer_port, leeched_files);
/// println!("{}", message);
/// ```
///
/// This will print:
///
/// ```
/// announce listen 8000 seed [file1.txt 100 10 abc123 file2.txt 200 20 def456] leech [file3.txt]
/// ```
pub fn seed(seeded: Vec<MetaFile>, peer_port: String, leeched: String) -> String {
    /*
    into_iter() : transform the vector into an iterator
    map() : apply a function to each element of the iterator
    collect() : transform the iterator into a vector
    join() : concatenate the elements of the vector into a single string
     */
    let seeded_string: Vec<String> = seeded
        .into_iter()
        .map(|file| {
            format!(
                "{} {} {} {}",
                file.file_name, file.length, file.piece_size, file.hash
            )
        })
        .collect();

    let msg = format!(
        "announce listen {} seed [{}] leech [{}]\r\n",
        peer_port,
        seeded_string.join(" "),
        leeched
    );
    msg
}
/// takes a filename and a filesize as String and format them into a look message
pub fn look(filename: String, filesize: String) -> String {
    let mut res: String = "look [".to_string();
    let mut b: bool = false;
    if !filename.is_empty() {
        res = format!("{}filename=\"{}\"", res, filename);
        b = true;
    }
    if !filesize.is_empty() {
        if b {
            res = format!("{} filesize{}", res, filesize);
        } else {
            res = format!("{}filesize{}", res, filesize);
        }
    }
    format!("{}]\n", res)
}

/// 
pub fn getfile_request(key: String) -> String {
    format!("getfile [{}]\n", key)
}
/// connects to a given adress and port
pub fn connect(port: u16, adress: &str) -> Option<TcpStream> {
    let stream = TcpStream::connect(format!("{}:{}", adress, port));
    match stream {
        Ok(stream) => {
            info!("Connected to {}:{}", adress, port);
            Some(stream)
        }
        Err(e) => {
            error!("Could not connect to tracker: {}", e);
            None
        }
    }
}
/// Sends a message to a given adress and port
pub fn send(stream: &mut TcpStream, message: String) {
    stream.write(message.as_bytes()).unwrap();
    info!("Sending to tracker: {}", message);
}
/// Receives a message from a given adress and port
pub fn receive(stream: &mut TcpStream) -> String {
    let mut buffer = [0; 1024];
    let port = stream.peer_addr().unwrap().port();
    let ip = stream.peer_addr().unwrap().ip();
    let mut reader = BufReader::new(stream);
    debug!("About to read from {}:{}", ip, port);
    match reader.read(&mut buffer) {
        Ok(_) => {
            info!(
                "Received from {}:{} {}",
                ip,
                port,
                String::from_utf8_lossy(&buffer)
            );
            buffer
                .iter()
                .map(|&c| char::from_u32(c as u32).unwrap())
                .collect::<String>()
        }
        Err(e) => {
            error!("Could not receive from tracker{}:{} {}", ip, port, e);
            String::from("")
        }
    }
}

pub fn update() -> String{
    
    let seeds : Vec<MetaFile> = get_seeding_files();
    let leeches : Vec<MetaFile> = get_leeching_files();
    
    let mut formated_seeds : String = String::new();
    let mut formated_leeches : String = String::new();

    let mut i : bool = false;

    for seed in seeds {
        let hash = seed.hash;
        formated_seeds += &hash;
        if i {
            formated_seeds += " ";
        }
        i = true;
    }

    i = false;

    for leech in leeches {
        let hash = leech.hash;
        formated_leeches += &hash;
        if i {
            formated_leeches += " ";
        }
        i = true;
    }

    format!("update seed [{}] leech [{}]\n", formated_seeds, formated_leeches)
}

#[cfg(test)]
mod tests {}
