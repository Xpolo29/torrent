use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

fn handle_client(mut stream: TcpStream) {
    // Set a read timeout of 10 seconds
    stream.set_read_timeout(Some(Duration::new(10, 0))).unwrap();

    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                // Connection was closed by the client
                break;
            }
            Ok(_) => {
                println!("Message received: {}", String::from_utf8_lossy(&buffer[..]));
                stream.write(&buffer).unwrap();
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // Timeout occurred
                break;
            }
            Err(e) => {
                // An error occurred
                eprintln!("Error: {}", e);
                break;
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        handle_client(stream.unwrap());
    }
}