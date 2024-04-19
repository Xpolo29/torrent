use crate::tasks::*;
use hashbrown::HashMap;
use log::error;
use regex::Regex;
use std::net::TcpStream;

//Enum for request types

enum RequestType {
    Data = 0,
    Have = 1,
    GetPieces = 2,
    Interested = 3,
}

/// This function takes a number and returns the corresponding RequestType.
/// If the number does not correspond to any RequestType, it returns None.
fn cast_to_request_type(number: u8) -> Option<RequestType> {
    match number {
        0 => Some(RequestType::Data),
        1 => Some(RequestType::Have),
        2 => Some(RequestType::GetPieces),
        3 => Some(RequestType::Interested),
        _ => None,
    }
}

pub enum Stream {
    Single(Option<TcpStream>),
    Multiple(Vec<Option<TcpStream>>),
}

/// Organizes a request based on its type.
///
/// # Arguments
/// * `re` - A Regex object used to parse the request.
/// * `request` - A String containing the request.
/// * `req_type` - The type of the request.
/// * `stream` - An optional TcpStream.
///
/// # Returns
/// * `Box<dyn Task + Send>` - A boxed Task object.
fn organize_request(
    re: Regex,
    request: String,
    req_type: RequestType,
    stream: Option<TcpStream>,
) -> Box<dyn Task + Send> {
    match req_type {
        RequestType::Data => data_request(re, request, stream),
        RequestType::Have => have_request(re, request, stream),
        RequestType::GetPieces => getpieces_request(re, request, stream),
        RequestType::Interested => interested_request(re, request, stream),
    }
}

/// This function takes a data request and returns a Task object that handles the request.
fn data_request(re: Regex, request: String, stream: Option<TcpStream>) -> Box<dyn Task + Send> {
    let regex_data = r"^(data) ([[:alnum:]]*) \\[((?:[[:digit:]]*:[01]* ?)*)\\]$";
    let capture = re.captures(&request).unwrap();
    let hash = capture.get(2).unwrap();
    let hashdata = capture.get(3).unwrap();
    let map: HashMap<u32, Vec<u8>> = hashdata
        .as_str()
        .split(' ')
        .map(|pair| {
            let (key, value) = pair.split_once(':').unwrap();
            let key: u32 = key.parse().unwrap();
            let value: Vec<u8> = value
                .chars()
                .filter_map(|c| u8::from_str_radix(&c.to_string(), 16).ok())
                .collect();
            (key, value)
        })
        .collect();
    let ret = Data {
        key: hash.as_str().to_string(),
        pieces: map.clone(),
        stream: stream,
    };
    println!("hash : {}, HashMap : {:?}", hash.as_str(), map);
    Box::new(ret)
}

/// This function takes a data request and returns a Task object that handles the request.
fn have_request(re: Regex, request: String, stream: Option<TcpStream>) -> Box<dyn Task + Send> {
    let capture = re.captures(&request).unwrap();
    let hash = capture.get(2).unwrap();
    let buffermap = capture.get(3).unwrap();
    let buf: Vec<u8> = buffermap
        .as_str()
        .chars()
        .map(|c| c.to_string())
        .map(|s| u8::from_str_radix(&s, 2).unwrap())
        .collect();
    let ret = Have {
        key: hash.as_str().to_string(),
        buffermap: buf,
        stream: stream,
    };
    Box::new(ret)
}

pub fn parse_data(request: String) -> Option<HashMap<u32, Vec<u8>>> {
    todo!();
}

pub fn parse_have_from_have(request: String, stream: Option<TcpStream>) -> Option<Have> {
    let regex_have = r"^(have) ([[:alnum:]]*) ([01]*)$";
    match Regex::new(regex_have) {
        Ok(re) => {
            let request_trimmed = request.trim().to_string();
            if re.is_match(&request_trimmed) {
                let capture = re.captures(&request).unwrap();

                let hash = capture.get(2).unwrap();
                let buffermap = capture.get(3).unwrap();
                let buf: Vec<u8> = buffermap
                    .as_str()
                    .chars()
                    .map(|c| c.to_string())
                    .map(|s| u8::from_str_radix(&s, 2).unwrap())
                    .collect();
                let ret = Have {
                    key: hash.as_str().to_string(),
                    buffermap: buf,
                    stream: stream,
                };
                Some(ret)
            } else {
                error!("Not a Have request");
                None
            }
        }
        Err(e) => {
            error!("Regex error: {}", e);
            None
        }
    }
}

/// This function takes a data request and returns a Task object that handles the request.
fn getpieces_request(
    re: Regex,
    request: String,
    stream: Option<TcpStream>,
) -> Box<dyn Task + Send> {
    let regex_getpieces = r"^(getpieces) ([[:alnum:]]*) \\[((?:[[:digit:]]* ?)*)\\]$";
    let capture = re.captures(&request).unwrap();
    let hash = capture.get(2).unwrap();
    let indexes = capture.get(3).unwrap();
    let numbers: Vec<u32> = indexes
        .as_str()
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let ret = Getpieces {
        key: hash.as_str().to_string(),
        pieces: numbers,
        stream: stream,
    };
    Box::new(ret)
}

/// This function takes a data request and returns a Task object that handles the request.
fn interested_request(
    re: Regex,
    request: String,
    stream: Option<TcpStream>,
) -> Box<dyn Task + Send> {
    let capture = re.captures(&request).unwrap();
    let hash = capture.get(2).unwrap();
    let ret = Interested {
        key: hash.as_str().to_string(),
        stream: stream,
    };
    let b: Box<dyn Task + Send> = Box::new(ret);
    b
}

/// This function takes a data request and returns a Task object that handles the request.
pub fn parse_request(request: String, stream: Option<TcpStream>) -> Box<dyn Task + Send> {
    // let empty = EmptyTask {
    //     stream: Some(stream),
    // };
    // return Box::new(empty);
    let regex_getpieces = r"^(getpieces) ([[:alnum:]]*) \\[((?:[[:digit:]]* ?)*)\\]$";
    let regex_interested = r"^(interested) ([[:alnum:]]*)$";
    let regex_have = r"^(have) ([[:alnum:]]*) ([01]*)$";
    let regex_data = r"^(data) ([[:alnum:]]*) \\[((?:[[:digit:]]*:[01]* ?)*)\\]$";
    let regex = [regex_data, regex_have, regex_getpieces, regex_interested];
    let mut count = 0;
    // let mut reqtype = RequestType::Data;
    /* */
    for r in regex {
        match Regex::new(r) {
            Ok(re) => {
                let request_trimmed = request.trim().to_string();
                if re.is_match(&request_trimmed) {
                    let reqtype = cast_to_request_type(count).unwrap();
                    return organize_request(re, request_trimmed, reqtype, stream);
                } else {
                    continue;
                };
            }
            Err(e) => {
                error!("Regex Error: {}", e);
            }
        }
        count += 1;
    }
    let empty = EmptyTask { stream: stream };
    Box::new(empty)
}

// Connect to the localhost
// pub fn parse_interested(request: String, stream: Option<TcpStream>) -> Box<dyn Task + Send> {
//     let regex_interested = r"^(interested) ([[:alnum:]]*)$";
//     let reg: Regex;
//     match Regex::new(regex_interested) {
//         Ok(re) => {
//             let request_trimmed = request.trim().to_string();
//             if re.is_match(&request_trimmed) {
//                 interested_request(re, request, stream)
//             } else {
//                 error!("Not an interested request");
//                 let empty = EmptyTask { stream: stream };
//                 Box::new(empty)
//             }
//         }
//         Err(e) => {
//             error!("Regex error : {}", e);
//             let empty = EmptyTask { stream: stream };
//             Box::new(empty)
//         }
//     }
// }
#[cfg(test)]
mod tests {
    use super::*;
    use env_logger::Builder;
    use std::io::Write;
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
}
