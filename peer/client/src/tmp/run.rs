use std::net::TcpStream;
mod send;
mod receive;
mod format;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8000").unwrap();
    println!("Connected");
    let message = format::format_input("Hello, Tracker!");
    send::send_message(&mut stream, &message).unwrap();
    println!("Sent");
    let response = receive::receive_message(&mut stream).unwrap();
    println!("Received: {}", response);
}