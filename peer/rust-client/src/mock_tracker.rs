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
                println!("Connection closed by peer");
                break;
            }
            Ok(_) => {
                let message = String::from_utf8_lossy(&buffer[..]);
                let command = message.split_whitespace().next().unwrap_or("");
                println!("Command received: {}", command);
                let response = match command {
                    "announce" => "ok\r\n",
                    "look" => "list [file_a.dat 2097152 1024 8905e92afeb80fc7722ec89eb0bf0966]\r\n",
                    "getfile" => {
                        "peers 8905e92afeb80fc7722ec89eb0bf0966 [1.1.1.2:2222 1.1.1.3:3333]\r\n"
                    }
                    _ => "Unknown command\r\n",
                };
                stream.write(response.as_bytes()).unwrap();
                println!("Response sent: {}", response);
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // Handle error
            }
            Err(e) => panic!("Unexpected error: {}", e),
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:12345").unwrap();
    for stream in listener.incoming() {
        handle_client(stream.unwrap());
    }
}
