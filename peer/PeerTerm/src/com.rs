//! communication between the peer and the tracker
use crate::data::MetaFile;
use crate::db::{get_leeching_files, get_seeding_files};
use core::cmp::min;
use log::{debug, error, info, warn};
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::net::TcpStream;
use std::time::Duration;
use crate::back::is_stream_open;

// format the data message
pub fn dataf(key: &String, pieces: Vec<String>) -> String {
    format!("data {} [{}]\n", key, pieces.join(" "))
}

// format the getpieces msg
pub fn getpiecesf(key: String, pieces: Vec<usize>) -> String {
    let indexes_str = pieces
        .iter()
        .map(|&index| index.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    format!("getpieces {} [{}]\n", key.trim(), indexes_str)
}

// format the interested message
pub fn interestedf(key: String) -> String {
    format!("interested {}\n", key)
}

// format the have msg
pub fn havef(key: String, buffermap: Vec<u8>) -> String {
    // convert [0, 0, 1, 0] to 0010
    let buffermap = buffermap
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("");

    let message: String = format!("have {} {}\n", key, buffermap);

    message
}

// # Examples
//
// ```
// let seeded_files = vec![
//     MetaFile {
//         file_name: "file1.txt".to_string(),
//         length: 100,
//         piece_size: 10,
//         hash: "abc123".to_string(),
//     },
//     MetaFile {
//         file_name: "file2.txt".to_string(),
//         length: 200,
//         piece_size: 20,
//         hash: "def456".to_string(),
//     },
// ];
// let peer_port = "8000".to_string();
// let leeched_files = "file3.txt".to_string();
// let message = seed(seeded_files, peer_port, leeched_files);
// println!("{}", message);
// ```
//
// This will print:
//
// ```
// announce listen 8000 seed [file1.txt 100 10 abc123 file2.txt 200 20 def456] leech [file3.txt]
// ```

/// Formats a seeding announcement message.
///
/// This function takes a vector of MetaFile objects seeded, a peer port, and a string leeched.
/// It formats a seeding announcement message by transforming the MetaFile objects into a string representation,
/// and then concatenating them into the announcement message.
///
/// # Arguments
/// * `seeded` - A vector of MetaFile objects that are being seeded.
/// * `peer_port` - A string representing the peer port.
/// * `leeched` - A string representing the leeched information.
///
/// # Returns
/// * `String` - The formatted seeding announcement message.
// format the seed message
pub fn seedf(seeded: Vec<MetaFile>, peer_port: String, leeched: Vec<String>) -> String {
    // why is seeded different from leached (type) ? Because of sujet
    /*
    into_iter() : transform the vector into an iterator
    map() : apply a function to each element of the iterator
    collect() : transform the iterator into a vector
    join() : concatenate the elements of the vector into a single string
    d
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
        leeched.join(" "),
    );
    msg
}

/// Formats a "look" message with a given filename and filesize.
///
/// This function takes a filename and a filesize as strings.
/// It formats a "look" message by concatenating the filename and filesize into the message.
///
/// # Arguments
/// * `filename` - A string representing the filename.
/// * `filesize` - A string representing the filesize.
///
/// # Returns
/// * `String` - The formatted "look" message.
// format the look message
pub fn lookf(filename: String, filesize: String) -> String {
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
            res = format!("{}filesize{}", res, filesize); // why do we add filesize if its empty ?
        }
    }
    format!("{}]\n", res)
}

/// Formats a "getfile" request message with a given key.
///
/// This function takes a key as a string and formats a "getfile" request message by inserting the key into the message.
///
/// # Arguments
/// * `key` - A string representing the key.
///
/// # Returns
/// * `String` - The formatted "getfile" request message.
// format the getfile message
pub fn getfilef(key: String) -> String {
    format!("getfile {}\n", key)
}

/// Establishes a TCP connection to a given address and port.
///
/// This function takes a port number and an address as arguments,
/// and attempts to establish a TCP connection to the specified address and port.
/// If the connection is successful, it returns an `Option` containing the `TcpStream`.
/// If the connection fails, it logs an error message and returns `None`.
///
/// # Arguments
/// * `port` - A u16 representing the port number.
/// * `adress` - A string slice representing the address.
///
/// # Returns
/// * `Option<TcpStream>` - The established TCP connection, or `None` if the connection failed.
pub fn connect(port: u16, adress: &str) -> Option<TcpStream> {
    let stream = TcpStream::connect(format!("{}:{}", adress, port));
    match stream {
        Ok(stream) => {
            info!("Connected to {}:{}", adress, port);
            Some(stream)
        }
        Err(e) => {
            error!("{} Could not connect to {}:{}", e, adress, port);
            None
        }
    }
}

/// Sends a message to a given address and port.
///
/// This function takes a mutable reference to a `TcpStream` and a message as a string.
/// It sends the message to the address and port associated with the `TcpStream`.
/// If the message is successfully sent, it logs an informational message.
///
/// # Arguments
/// * `stream` - A mutable reference to a `TcpStream`.
/// * `message` - A string representing the message to be sent.
pub fn send(stream: &mut TcpStream, message: String) {

    if !is_stream_open(stream) {
        warn!("Trying to send to closed stream");
        return;
    }
    stream.write(message.as_bytes()).unwrap();
    debug!(
        "Sending to {} : {}",
        stream.peer_addr().unwrap(),
        message.chars().take(128).collect::<String>()
    );
}

/// Receives a message from a given address and port.
///
/// This function takes a mutable reference to a `TcpStream`.
/// It reads a message from the address and port associated with the `TcpStream` into a buffer.
/// If the message is successfully read, it logs an informational message and returns the message as a string.
/// If the message cannot be read, it logs an error message and returns an empty string.
///
/// # Arguments
/// * `stream` - A mutable reference to a `TcpStream`.
///
/// # Returns
/// * `String` - The message received from the `TcpStream`, or an empty string if the message could not be read.
pub fn receive(stream: &mut TcpStream, timeout_ms: u64) -> String {
    if !is_stream_open(stream) {
        warn!("Trying to receive from closed stream");
        return "".to_string();
    }

    let mut buffer: Vec<u8> = Vec::new();
    let port = stream.peer_addr().unwrap().port();
    let ip = stream.peer_addr().unwrap().ip();
    let mut reader = BufReader::new(stream);
    debug!("About to read from {}:{}", ip, port);
    // implement timeout so that this method doesnt block, 1s timeout
    reader
        .get_ref()
        .set_read_timeout(Some(Duration::from_millis(timeout_ms)))
        .unwrap();

    loop {
        match reader.read_until(b'\n', &mut buffer) {
            Ok(0) => {
                // Timeout occurred, check if any data was read
                if buffer.is_empty() {
                    // No data was read, continue with an empty buffer
                    if timeout_ms > 2000 {
                        warn!("Didn't receive any data from {}:{}", ip, port);
                    }
                    return "".to_string();
                } else {
                    // Data was read, continue processing the buffer
                    debug!("Msg partially read, will continue reading");
                }
            }
            Ok(_) => {
                debug!(
                    "Received from {}:{} {}",
                    ip,
                    port,
                    String::from_utf8_lossy(&buffer[0..min(128, buffer.len())]) // only shows the first 128 chars
                );
                return buffer
                    .iter()
                    .map(|&c| char::from_u32(c as u32).unwrap())
                    .collect::<String>();
            }
            Err(e) => {
                if e.kind() == ErrorKind::WouldBlock {
                    // Timeout occurred, check if any data was read
                    if buffer.is_empty() {
                        // No data was read, continue with an empty buffer
                        if timeout_ms > 2000 {
                            warn!("Didn't receive any data from {}:{}", ip, port);
                        }
                        return "".to_string();
                    } else {
                        // Data was read, continue processing the buffer
                        debug!("Msg partially read, will continue reading");
                    }
                } else {
                    error!("Could not receive from {}:{} because of {}", ip, port, e);
                    return "".to_string();
                }
            }
        }
    }
}

/// Generates an update message with the current seeding and leeching files.
///
/// This function retrieves the list of seeding and leeching files.
/// It formats these lists into strings, where each file is represented by its hash and files are separated by spaces.
/// It then generates an update message containing these formatted lists of seeding and leeching files.
///
/// # Returns
/// * `String` - The update message containing the formatted lists of seeding and leeching files.
// format the update message
pub fn updatef() -> String {
    let seeds: Vec<MetaFile> = get_seeding_files();
    let leeches: Vec<MetaFile> = get_leeching_files();

    let mut formated_seeds: String = String::new();
    let mut formated_leeches: String = String::new();

    let mut i: bool = false;

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

    format!(
        "update seed [{}] leech [{}]\n",
        formated_seeds, formated_leeches
    )
}

#[cfg(test)]
mod tests {}
