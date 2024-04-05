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

fn cast_to_request_type(number: u8) -> Option<RequestType> {
    match number {
        0 => Some(RequestType::Data),
        1 => Some(RequestType::Have),
        2 => Some(RequestType::GetPieces),
        3 => Some(RequestType::Interested),
        _ => None,
    }
}

fn organize_request(
    re: Regex,
    request: String,
    req_type: RequestType,
    stream: TcpStream,
) -> Box<dyn Task + Send> {
    match req_type {
        RequestType::Data => data_request(re, request, stream),
        RequestType::Have => have_request(re, request, stream),
        RequestType::GetPieces => getpieces_request(re, request, stream),
        RequestType::Interested => interested_request(re, request, stream),
    }
}

fn data_request(re: Regex, request: String, stream: TcpStream) -> Box<dyn Task + Send> {
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
        pieces: map,
        stream: Some(stream),
    };
    Box::new(ret)
}
fn have_request(re: Regex, request: String, stream: TcpStream) -> Box<dyn Task + Send> {
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
        stream: Some(stream),
    };
    Box::new(ret)
}
fn getpieces_request(re: Regex, request: String, stream: TcpStream) -> Box<dyn Task + Send> {
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
        stream: Some(stream),
    };
    Box::new(ret)
}
fn interested_request(re: Regex, request: String, stream: TcpStream) -> Box<dyn Task + Send> {
    let capture = re.captures(&request).unwrap();
    let hash = capture.get(2).unwrap();
    let ret = Interested {
        key: hash.as_str().to_string(),
        stream: Some(stream),
    };
    let b: Box<dyn Task + Send> = Box::new(ret);
    b
}
pub fn parse_request(request: String, stream: TcpStream) -> Box<dyn Task + Send> {
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
    let empty = EmptyTask {
        stream: Some(stream),
    };
    return Box::new(empty);
}
