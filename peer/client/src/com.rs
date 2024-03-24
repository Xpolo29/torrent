use crate::data::MetaFile;
use log::{debug, error, info};
use std::io::{BufReader, Read, Write};
use std::net::TcpStream;
/// Takes a list of seeded and leeched files with medata data and returns the right message to be sent
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
pub fn look(criterion: String) -> String {
    format!("look [filename=\"{}\"]\n", criterion)
}
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

#[cfg(test)]
mod tests {}
