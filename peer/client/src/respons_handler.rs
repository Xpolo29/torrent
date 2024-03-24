use crate::data::MetaFile;
use log::error;
use regex::Regex;
use std::io;
use std::net::TcpStream;
pub trait ExpectedAnswer {
    // Check if the answer is correctly formatted
    fn check_answer(&self, answer: &str) -> Result<String, std::io::Error>;
    // Retrieve the relevant data from the answer returns an Answer enum which convey right data type
    fn retrieve_data(&self, answer: String) -> Result<Answer, std::io::Error>;
    // Shutdown the stream so that it does ping pong style communication
    fn shutdown(&self, stream: &mut TcpStream);
}
impl ExpectedAnswer for ExpectOk {
    fn check_answer(&self, answer: &str) -> Result<String, io::Error> {
        let re = Regex::new(r"^ok$").unwrap();
        let first_line = answer.lines().next().unwrap_or("");
        if re.is_match(first_line) {
            Ok("Correct tracker answer".to_string())
        } else {
            error!("Failed tracker answer: {}", answer);
            Err(io::Error::new(io::ErrorKind::Other, "Bad tracker answer"))
        }
    }
    fn retrieve_data(&self, _answer: String) -> Result<Answer, std::io::Error> {
        Ok(Answer::Ok)
    }

    fn shutdown(&self, stream: &mut TcpStream) {
        stream.shutdown(std::net::Shutdown::Both).unwrap();
    }
}
impl ExpectedAnswer for ExpectList {
    fn check_answer(&self, answer: &str) -> Result<String, std::io::Error> {
        let re = Regex::new(r"^list \[((?:\w+\.\w+ \d+ \d+ \w+ )*)\]$").unwrap();
        if re.is_match(answer) {
            Ok("Correct tracker answer".to_string())
        } else {
            error!("Failed tracker answer: {}", answer);
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Bad tracker answer",
            ))
        }
    }
    fn retrieve_data(&self, answer: String) -> Result<Answer, std::io::Error> {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Not implemented",
        ))
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
pub struct ExpectList;
