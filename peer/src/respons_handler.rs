use crate::data::{MetaFile, PeerConfig};
use log::{error, trace};

use regex::Regex;
use std::error::Error;
use std::io;
use std::net::TcpStream;
pub trait ExpectedAnswer {
    // Check if the answer is correctly formatted
    fn check_answer(&self, answer: &str) -> Result<String, Box<dyn Error>>;
    // Retrieve the relevant data from the answer returns an Answer enum which convey right data type
    fn retrieve_data(&self, answer: String) -> Answer;
    // Shutdown the stream so that it does ping pong style communication
    fn shutdown(&self, stream: &mut TcpStream);
}
impl ExpectedAnswer for ExpectOk {
    fn check_answer(&self, answer: &str) -> Result<String, Box<dyn Error>> {
        match Regex::new(r"^ok$") {
            Ok(re) => {
                let first_line = answer.lines().next().unwrap_or("");
                if re.is_match(first_line) {
                    Ok("Correct tracker answer".to_string())
                } else {
                    error!("Failed tracker answer: {}", answer);
                    Err(Box::new(io::Error::new(
                        io::ErrorKind::Other,
                        "Bad tracker answer",
                    )))
                }
            }
            Err(e) => {
                error!("Regex error: {}", e);
                Err(Box::new(e))
            }
        }
    }
    fn retrieve_data(&self, _answer: String) -> Answer {
        Answer::Ok
    }

    fn shutdown(&self, stream: &mut TcpStream) {
        stream.shutdown(std::net::Shutdown::Both).unwrap();
    }
}
impl ExpectedAnswer for ExpectList {
    fn check_answer(&self, answer: &str) -> Result<String, Box<dyn Error>> {
        //match Regex::new(r"^list \[(\S+ \d+ \d+ \w+ ?)*\] ?((\u{000A})?(\u{000D})?(\u{0000})*)?$") {
        match Regex::new(r"^list \[(\S+ \d+ \d+ \w+ ?)*\] ?$") {
            Ok(re) => {
                let first_line = answer.lines().next().unwrap_or("");
                trace!("Answer to be checked: {}", first_line);
                if re.is_match(first_line) {
                    Ok("Correct tracker answer".to_string())
                } else {
                    error!("Failed tracker answer: {}", answer);
                    for c in first_line.chars() {
                        trace!("U+{:04X} {}", c as u32, c)
                    }
                    Err(Box::new(io::Error::new(
                        io::ErrorKind::Other,
                        "Bad tracker answer",
                    )))
                }
            }
            Err(e) => {
                error!("Regex error: {}", e);
                Err(Box::new(e))
            }
        }
    }
    // precond : answer is a valid list answer
    fn retrieve_data(&self, answer: String) -> Answer {
        let answer = answer.trim().to_string();
        trace!("Answer to be retrieved: {}", answer);
        let answer = &answer[6..answer.len() - 1]; // Remove "list [" and "]"
        let re_file =
            Regex::new(r"(?P<file_name>\S+) (?P<length>\d+) (?P<piece_size>\d+) (?P<hash>\w+)")
                .unwrap();
        let mut files = Vec::new();
        for caps in re_file.captures_iter(answer) {
            let file = MetaFile {
                file_name: caps["file_name"].to_string(),
                length: caps["length"].parse().unwrap(),
                piece_size: caps["piece_size"].parse().unwrap(),
                hash: caps["hash"].to_string(),
            };
            files.push(file);
        }
        Answer::List(files)
    }

    fn shutdown(&self, stream: &mut TcpStream) {
        stream.shutdown(std::net::Shutdown::Both).unwrap();
    }
}

impl ExpectedAnswer for ExpectPeers {
    fn check_answer(&self, answer: &str) -> Result<String, Box<dyn Error>> {
        todo!()
    }
    fn retrieve_data(&self, answer: String) -> Answer {
        todo!()
    }
    fn shutdown(&self, stream: &mut TcpStream) {
        todo!()
    }
}

#[derive(Debug)]
pub enum Answer {
    Ok,
    List(Vec<MetaFile>),
    Peers(Vec<PeerConfig>),
}
pub struct ExpectOk;
pub struct ExpectList;
pub struct ExpectPeers;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_answer_with_list_trait() {
        let answer = "list [file_a.dat 2097152 1024 8905e92afeb80fc7722ec89eb0bf0966]\r\n";
        let expect_list = ExpectList;

        let result = expect_list.check_answer(answer);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Correct tracker answer");
    }

    #[test]
    fn test_check_answer_with_list_trait_additional_elements() {
        let answer = "list [file_a.dat 2097152 1024 8905e92afeb80fc7722ec89eb0bf0966 file_b.dat 2097152 1024 8905e92afeb80fc7722ec89eb0bf0966 file_c.dat 2097152 1024 8905e92afeb80fc7722ec89eb0bf0966 file_d.dat 2097152 1024 8905e92afeb80fc7722ec89eb0bf0966]\r\n";
        let expect_list = ExpectList;

        let result = expect_list.check_answer(answer);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Correct tracker answer");
    }
}
