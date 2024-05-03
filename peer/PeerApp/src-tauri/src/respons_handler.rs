use crate::data::{MetaFile, PeerConfig};
use crate::tasks::Peer;
use log::{error, trace};
use regex::Regex;
use std::error::Error;
use std::io;
use std::net::TcpStream;
use crate::threads::Pool;


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
        // TODO Correct chech instead of always true
        let res: String = String::from(answer);
        Ok(res)
    }
    fn retrieve_data(&self, answer: String) -> Answer {
        let answer = answer.trim().to_string();
        trace!("Answer to be retrieved: {}", answer);
        // get hash
        let key: String = String::from(&answer[6..38]);
        // Remove "peers %hash% [" and "]"
        let answer = &answer[39..answer.len()];
        let mut peers: Vec<Peer> = Vec::new();

        // fill the peers array
        let reg = Regex::new(r"\[(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}):(\d+)\]").unwrap();

        for peer in reg.captures_iter(answer) {
            let myself = PeerConfig::from_config();
            let address = peer[1].to_string();
            let port = peer[2].parse::<u16>().unwrap();
            // trace!("Succefully captured peer : {}:{}", address, port);
            let config: PeerConfig = PeerConfig { address, port };
            if myself.address == config.address && myself.port == config.port {
                continue;
            }
            let hash: String = key.clone();
            // trying to init with an empty pool
            let pool: Pool = Pool::new(0); 
            peers.push(Peer { hash, config, pool });
        }

        Answer::Peers(peers)
    }
    fn shutdown(&self, stream: &mut TcpStream) {
        todo!()
    }
}

impl ExpectedAnswer for ExpectData{
    fn check_answer(&self, answer: &str) -> Result<String, Box<dyn Error>>{
        // TODO Correct chech instead of always true
        let res: String = String::from(answer);
        Ok(res)
    }
    fn retrieve_data(&self, answer: String) -> Answer{
        // TODO use regex instead of string splitting
        let mut map: Vec<(usize, Vec<u8>)> = Vec::new();

        let answer: String = answer.trim().to_string();

        let datas: Vec<&str> = answer.as_str().split('[').collect();
        let datas: Vec<&str> = datas[1].split(']').collect();
        let datas: Vec<&str> = datas[0].split(' ').collect();

        for data in datas{
            let splitted: Vec<&str> = data.split(':').collect();
            let key: usize = splitted[0].parse().unwrap();
            let data_str: String = splitted[1].to_string();

            // trace!("Key : {}, Value : {}", key, data_str);

            // convert data string into u8, using 8 bits chunks
            let mut pieces: Vec<u8> = Vec::new();
            for chunk in data_str.as_bytes().chunks(8) {
                let chunk_str = std::str::from_utf8(chunk).unwrap();
                let num = u8::from_str_radix(chunk_str, 2).unwrap(); //2 means base 2 binary to u8
                trace!("Num : {}", num);
                pieces.push(num);
            }

            map.push((key, pieces));
        }
        Answer::Data(map)
    }
    fn shutdown(&self, stream: &mut TcpStream){
        todo!();
    }
}


#[derive(Debug)]
pub enum Answer {
    Ok,
    List(Vec<MetaFile>),
    Peers(Vec<Peer>),
    Data(Vec<(usize, Vec<u8>)>),    
}
pub struct ExpectOk;
pub struct ExpectList;
pub struct ExpectPeers;
pub struct ExpectData;
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
