use std::net::TcpStream;
use std::io::{Read, Write};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    let msg = b"Hello, server!";
    stream.write(msg).unwrap();
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
}   