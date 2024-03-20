use crate::data::{MetaFile, PeerConfig, TrackerConfig};
use log::{error, info, warn};
use std::io::{BufRead, BufReader, Write};
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
/// Sends a message to a given adress and port
pub fn send(message: String, port: u16, adress: String) {
    let mut stream = TcpStream::connect(format!("{}:{}", adress, port));
    match stream {
        Ok(mut stream) => {
            stream.write(message.as_bytes()).unwrap();
            info!("Sending to {}:{} < {}", adress, port, message);
        }
        Err(e) => {
            error!("Could not connect to tracker: {}", e);
        }
    }
}

/// Receives a message from a given adress and port
pub fn receive(expected_answer: &dyn ExpectedAnswer, port: u16, adress: String) -> String {
    let mut stream = TcpStream::connect(format!("{}:{}", adress, port));
    match stream {
        Ok(mut stream) => {
            let mut buffer = String::new();
            let mut reader = BufReader::new(&stream);
            match reader.read_line(&mut buffer) {
                Ok(_) => {
                    info!("Received from {}:{} > {}", adress, port, buffer);
                    expected_answer.shutdown(&mut stream);
                    return buffer;
                }
                Err(e) => {
                    error!("Could not receive from tracker: {}", e);
                    expected_answer.shutdown(&mut stream);
                    return String::from("");
                }
            }
        }
        Err(e) => {
            error!("Could not connect to tracker: {}", e);
            return String::from("");
        }
    }
}

pub trait ExpectedAnswer {
    fn check_answer(&self, answer: &str) -> Result<String, std::io::Error>;
    fn retrieve_data(&self, answer: String) -> Result<Answer, std::io::Error>;
    fn shutdown(&self, stream: &mut TcpStream);
}

impl ExpectedAnswer for ExpectOk {
    fn check_answer(&self, answer: &str) -> Result<String, std::io::Error> {
        if answer == "ok\n" {
            Ok("Correct tracker answer".to_string())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Bad tracker answer",
            ))
        }
    }
    fn retrieve_data(&self, _answer: String) -> Result<Answer, std::io::Error> {
        Ok(Answer::Ok)
    }

    fn shutdown(&self, stream: &mut TcpStream) {
        stream.shutdown(std::net::Shutdown::Both).unwrap();
    }
}

pub enum Answer {
    Ok,
    List(Vec<MetaFile>),
}
pub struct ExpectOk;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seed() {
        // Arrange
        let seeded = vec![
            MetaFile {
                file_name: String::from("file1.txt"),
                length: 100,
                piece_size: 10,
                hash: String::from("hash1"),
            },
            MetaFile {
                file_name: String::from("file2.txt"),
                length: 200,
                piece_size: 20,
                hash: String::from("hash2"),
            },
        ];
    }
}
