use std::collections::HashMap;
use std::net::TcpStream;
use regex::Regex;
use crate::parser::{data_request, RequestType, cast_to_request_type};
use crate::process::Task;
use super::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::Read;

fn main() {
    let re = Regex::new(r"^(data) ([[:alnum:]]*) \\[((?:[[:digit:]]*:[01]* ?)*)\\]$").unwrap();
    let request = String::from("8905e92afeb80fc7722ec89eb0bf0966 [3:%piece3% 5:%piece5% 7:%piece7% 8:%piece8% 9:%piece9%]");
    let stream = TcpStream::connect("127.0.0.1:9090").ok();

    let task = data_request(re, request, stream);
    task.process();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{get_buffermap, get_peer_key, set_buffermap};
    use std::net::{TcpListener, TcpStream};
    use env_logger::Builder;
    use std::io::Write;

    #[test]
    fn test_cast_to_request_type() {
        assert_eq!(cast_to_request_type(0), Some(RequestType::Data));
        assert_eq!(cast_to_request_type(1), Some(RequestType::Have));
        assert_eq!(cast_to_request_type(2), Some(RequestType::GetPieces));
        assert_eq!(cast_to_request_type(3), Some(RequestType::Interested));
        assert_eq!(cast_to_request_type(4), None);
    }

    #[test]
    fn test_getpieces_process() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let key = "test_key".to_string();
        let pieces = vec![0, 1, 2];

        let mut getpieces = Getpieces {
            stream: None,
            key: key.clone(),
            pieces: pieces.clone(),
        };

        thread::spawn(move || {
            getpieces.stream = Some(TcpStream::connect(("127.0.0.1", port)).unwrap());
            getpieces.process();
        });

        let (mut stream, _) = listener.accept().unwrap();
        let mut buffer = String::new();
        stream.read_to_string(&mut buffer).unwrap();

        assert!(buffer.contains(&key));
        for piece in pieces {
            assert!(buffer.contains(&piece.to_string()));
        }
    }

    #[test]
    fn test_data_process() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let key = "test_key".to_string();
        let pieces = HashMap::new(); // should add some test data 

        let mut data = Data {
            stream: None,
            key: key.clone(),
            pieces: pieces.clone(),
        };

        thread::spawn(move || {
            data.stream = Some(TcpStream::connect(("127.0.0.1", port)).unwrap());
            data.process();
        });

        let (mut stream, _) = listener.accept().unwrap();
        let mut buffer = String::new();
        stream.read_to_string(&mut buffer).unwrap();

        assert_eq!(buffer, "ok\n");
    }
}


    #[test]
    fn init_logger() {
        Builder::new()
            .format(|f, record| writeln!(f, "{}: {}", record.level(), record.args()))
            .init();
    }

    #[test]
    fn test_data_request() {
        let req = "data av12 [3:110011]";
        let stream_option = create_dummy_tcp_stream();
        parse_request(req.to_string(), stream_option);
    }

    fn create_dummy_tcp_stream() -> Option<TcpStream> {
        let stream = match TcpStream::connect("127.0.0.1:8000") {
            Ok(s) => s,
            Err(_) => return None,
        };

        // Close the connection
        if let Err(_) = stream.shutdown(std::net::Shutdown::Both) {
            return None;
        }

        Some(stream)
    }
}
