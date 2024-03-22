use crate::data::{MetaFile, PeerConfig, TrackerConfig};
use log::{debug, error, info, warn};
use std::io::{BufRead, BufReader, Write, Read};
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
pub fn connect(port:u16, adress:&str) -> Result<TcpStream, std::io::Error>{
    let mut stream = TcpStream::connect(format!("{}:{}", adress, port));
    match stream {
        Ok(mut stream) => {
            info!("Connected to {}:{}", adress, port);
            return Ok(stream);
        }
        Err(e) => {
            error!("Could not connect to tracker: {}", e);
            return Err(e);
        }
    }
}
/// Sends a message to a given adress and port
pub fn send(stream: &mut TcpStream, message:String) {
    stream.write(message.as_bytes()).unwrap();
    info!("Sending to tracker: {}", message);
}
/// Receives a message from a given adress and port
pub fn receive(expected_answer: &dyn ExpectedAnswer, mut stream:TcpStream) -> String {
    let mut buffer = [0; 1024];
    let mut reader = BufReader::new(&stream);
    debug!("About to read from {}:{}", stream.peer_addr().unwrap().ip(), stream.peer_addr().unwrap().port());
    match reader.read(&mut buffer) {
        Ok(_) => {
            info!("Received from tracker: {}", String::from_utf8_lossy(&buffer));
            expected_answer.shutdown(&mut stream);
            buffer.iter().map(|&c| char::from_u32(c as u32).unwrap()).collect::<String>()
        }
        Err(e) => {
            error!("Could not receive from tracker: {}", e);
            expected_answer.shutdown(&mut stream);
            String::from("")
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
    use std::io::Write;
    use std::net::{TcpListener, TcpStream};
    use std::thread;

    #[test]
    fn test_receive() {
        // Start a mock server in a new thread
        let handle = thread::spawn(|| {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let port = listener.local_addr().unwrap().port();
            let (mut stream, _) = listener.accept().unwrap();
            write!(stream, "ok\n").unwrap();
            port
        });

        // Get the port that the mock server is listening on
        let port = handle.join().unwrap();

        // Test the receive function
        let answer = receive(&ExpectOk, port, "127.0.0.1".to_string());
        assert_eq!(answer, "ok\n");
    }
}
