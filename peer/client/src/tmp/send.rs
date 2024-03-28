use std::net::TcpStream;
use std::io::Write;

pub fn send_message(stream: &mut TcpStream, message: &str) -> std::io::Result<()> {
    stream.write_all(message.as_bytes())
}