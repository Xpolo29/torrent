use crate::tasks::*;
use hashbrown::HashMap;
use log::{error, trace, info};
use regex::Regex;
use std::net::TcpStream;
use crate::threads::Pool;
use crate::data::MetaFile;
use crate::db::get_file;
use std::cmp::min;

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
    pool: Pool,
) -> Box<dyn Task + Send> {
    match req_type {
        RequestType::Data => data_request(re, request, stream),
        RequestType::Have => have_request(re, request, stream),
        RequestType::GetPieces => getpieces_request(re, request, stream, pool),
        RequestType::Interested => interested_request(re, request, stream),
    }
}

/// This function takes a data request and returns a Task object that handles the request.
fn data_request(re: Regex, request: String, stream: Option<TcpStream>) -> Box<dyn Task + Send> {
    info!("Received data request");
    trace!("Regex data matched");
    let capture = re.captures(&request).unwrap();
    let hash = capture.get(2).unwrap();
    let datas = capture.get(3).unwrap();

    let mut map: Vec<(usize, Vec<u8>)> = Vec::new();

    let datas_iter = datas.as_str().split(' ');

    for data in datas_iter{
        let splitted: Vec<&str> = data.split(':').collect();
        let key: usize = splitted[0].parse().unwrap();
        let data_str: String = splitted[1].to_string();

        // trace!("Key : {}, Value : {}", key, data_str);

        // convert data string into u8, using 8 bits chunks
        let mut pieces: Vec<u8> = Vec::new();
        for chunk in data_str.as_bytes().chunks(8) {
            let chunk_str = std::str::from_utf8(chunk).unwrap();
            let num = u8::from_str_radix(chunk_str, 2).unwrap(); //2 means base 2 binary to u8
            pieces.push(num);
        }

        map.push((key, pieces));
    }

    let ret = Data {
        key: hash.as_str().to_string(),
        pieces: map.clone(),
        stream: stream,
    };
    // let ret = EmptyTask {stream : None};
    // trace!("hash : {}, data : {:?}", hash.as_str(), map);
    Box::new(ret)

}

/// This function takes a data request and returns a Task object that handles the request.
fn have_request(re: Regex, request: String, stream: Option<TcpStream>) -> Box<dyn Task + Send> {
    info!("Received have request");
    trace!("Regex have matched");
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
    info!("Received data request");
    let regex_data = r"^(data) ([[:alnum:]]*) \\[((?:[[:digit:]]*:[01]* ?)*)\\]$";
    match Regex::new(regex_data) {
        Ok(re) => {
            let request_trimmed = request.trim().trim_matches(&['\0', '\n', ' '] as &[_]).to_string();
            if re.is_match(&request_trimmed) {
                let capture = re.captures(&request).unwrap();
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
                Some(map)
            } else {
                error!("Could not parse request as data: {}", &request[..min(128, request.len())]);
                None
            }
        }
        Err(e) => {
            error!("Regex error: {}", e);
            None
        }
    }
}


pub fn parse_have_from_have(request: String) -> Option<Have> {
    info!("Received have request");
    if request.len() == 0{
        return None;
    }
    let regex_have = r"^(have) ([[:alnum:]]*) ([01]*)$";
    match Regex::new(regex_have) {
        Ok(re) => {
            let request_trimmed = request.trim().trim_matches(&['\0', '\n', ' '] as &[_]).to_string();
            if re.is_match(&request_trimmed) {
                let capture = re.captures(&request_trimmed).unwrap();

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
                    stream: None,
                };
                Some(ret)
            } else {
                error!("Could not parse request as have: {}", &request[..min(128, request.len())]);
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
    pool: Pool,
) -> Box<dyn Task + Send> {
    trace!("Regex getpiece matched");
    //info!("Received getpieces request");
    //let regex_getpieces = r"^(getpieces) ([[:alnum:]]*) \\[((?:[[:digit:]]* ?)*)\\]$";
    let capture = re.captures(&request).unwrap();
    let hash = capture.get(2).unwrap();
    let indexes = capture.get(3).unwrap();
    let numbers: Vec<usize> = indexes
        .as_str()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    // trace!("Getpiece parser caught these : {:?}", numbers);
    let chunk_size: usize;
    let file_option: Option<MetaFile> = get_file(hash.as_str());
    match file_option {
        Some(file) => chunk_size = file.piece_size,
        None => chunk_size = 1024, 
    }
    let ret = Getpieces {
        key: hash.as_str().to_string(),
        chunk_size: chunk_size,
        pieces: numbers,
        stream: stream,
        pool: pool,
        retry: 0,
    };
    Box::new(ret)
}

/// This function takes a data request and returns a Task object that handles the request.
fn interested_request(
    re: Regex,
    request: String,
    stream: Option<TcpStream>,
) -> Box<dyn Task + Send> {
    trace!("Regex interest matched");
    info!("Received interested request");
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
pub fn parse_request(request: String, stream: Option<TcpStream>, pool: Pool) -> Box<dyn Task + Send> {
    // let empty = EmptyTask {
    //     stream: Some(stream),
    // };
    // return Box::new(empty);
    // let regex_getpieces = r"^(getpieces) ([[:alnum:]]*) \\[((?:[[:digit:]]* ?)*)\\]$";
    let regex_getpieces = r"^(getpieces) ([[:alnum:]]*) \[((?:[[:digit:]]* ?)*)\]$";
    // let regex_getpieces = r"^(getpieces) ([[:alnum:]]*) \[\s*(\d+\s*)*\s*\]$";
    let regex_interested = r"^(interested) ([[:alnum:]]*)$";
    //let regex_have = r"^(have) ([[:alnum:]]*) \[((?:[[:digit:]]*)*)\]$";
    let regex_have = r"^(have) ([[:alnum:]]*) ([01]*)$";
    let regex_data = r"^(data) ([[:alnum:]]*) \[((?:[[:digit:]]*:[01]* ?)*)\]$";
    let regex = [regex_data, regex_have, regex_getpieces, regex_interested];
    let mut count = 0;
    // let mut reqtype = RequestType::Data;
    for r in regex {
        match Regex::new(r) {
            Ok(re) => {
                let request_trimmed = request.trim().trim_matches(&['\0', '\n', ' '] as &[_]).to_string();
                /*
                trace!("request trimmed look like this : {}", request_trimmed);
                for c in request_trimmed.chars(){
                    println!("-> {} : {}", c, c as u32);
                }
                */
                if re.is_match(&request_trimmed) {
                    let reqtype = cast_to_request_type(count).unwrap();
                    return organize_request(re, request_trimmed, reqtype, stream, pool);
                } else {
                    count += 1;
                    continue;
                };
            }
            Err(e) => {
                error!("Regex Error: {}", e);
            }
        }

    }
    error!("Request error, could not match incoming request: {}", &request[..128]);
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
        parse_request(req.to_string(), stream_option, Pool::new(0));
    }
}
