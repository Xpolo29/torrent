use std::net::TcpStream;
use std::io::Read;

pub fn receive_message(stream: &mut TcpStream) -> std::io::Result<String> {
    let mut buffer = [0; 1024];
    let size = stream.read(&mut buffer)?;
    Ok(String::from_utf8_lossy(&buffer[..size]).to_string())
}