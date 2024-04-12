use std::io::prelude::*;
use std::net::TcpStream;

struct MetaFile {
    filename: String,
    length: usize,
    piece_size: usize,
    key: String,
}

// Function that takes two vectors of MetaFile and a port as input and returns a string
fn announce_listen_seed_leech(port: u16, seed: Vec<MetaFile>, leech: Vec<String>) -> String {
    let seed_str = seed.into_iter()
        .map(|file| format!("{} {} {} {}", file.filename, file.length, file.piece_size, file.key))
        .collect::<Vec<String>>()
        .join(" ");
    let leech_str = leech.join(" ");
    format!("< announce listen {} seed [{}] leech [{}]\n> ok", port, seed_str, leech_str)
}

// Function that takes a vector of strings as input and returns a string
fn look(criterion: Vec<String>) -> String {
    let criterion_str = criterion.join(" ");
    format!("< look [{}]\n> list", criterion_str)
}

// Function that takes a string as input and returns a string
fn getfile(key: String) -> String {
    format!("< getfile {}\n> peers", key)
}

// Function that takes a string as input and returns a string
fn interested(key: String) -> String {
    format!("< interested {}\n> have", key)
}

// Function that takes a string and a vector of strings as input and returns a string
fn getpieces(key: String, indices: Vec<String>) -> String {
    let indices_str = indices.join(" ");
    format!("< getpieces {} [{}]\n> data", key, indices_str)
}

// Function that takes a string and an ip address and a port, and send a tcp message with the string to the address and port
fn send_tcp_message(message: String, ip: String, port: u16) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(format!("{}:{}", ip, port))?;
    stream.write_all(message.as_bytes())?;
    Ok(())
}