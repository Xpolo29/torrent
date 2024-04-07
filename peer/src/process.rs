use crate::com::send;
use crate::data::PeerConfig;
use crate::db::get_buffermap;
use crate::tasks::{Data, Getpieces, Have, Interested, Task};
use hex;
use log::{error, trace};
impl Task for Interested {
    fn process(&mut self) {
        println!("This is an interested request");
    }
}
/// write a data to TCP
impl Task for Getpieces {
    fn process(&mut self) {
        // get key from getpieces
        let key = &self.key;
        // get the indexes of each piece
        let piece_indexes = &self.pieces;
        // get the stream
        let stream = &mut self.stream;
        // construct a peerconfig to pass to get_buffermap
        let port = stream.as_ref().unwrap().peer_addr().unwrap().port();
        let ip = stream.as_ref().unwrap().peer_addr().unwrap().ip();
        let peerconfig = PeerConfig {
            address: ip.to_string(),
            port,
        };
        // get the buffermap from the database
        let buffermap = get_buffermap(peerconfig, key);
        let chunk_size = 1024 * 8;
        match stream {
            Some(stream) => {
                match buffermap {
                    Some(buffermap) => {
                        let mut result = Vec::new();
                        // iterate over the piece indexes
                        for (i, piece_index) in piece_indexes.iter().enumerate() {
                            // calculate the start and end of the chunk with regard to the index
                            let start = (*piece_index as usize) * chunk_size;
                            let end = start + chunk_size;
                            if end <= buffermap.len() {
                                // get the slice of the buffermap
                                let slice = &buffermap[start..end];
                                result.push((i, slice.to_vec()));
                            } else {
                                error!("Buffermap is not large enough for piece {}", piece_index);
                            }
                            let formated_result = result
                                .iter()
                                .map(|(index, piece)| format!("{}:{} ", index, hex::encode(piece))) // encode it in hexadecimal
                                .collect::<String>();
                            let data = format!("data {} [{}]", key, formated_result);
                            trace!("Sending this data to the client: {:?}", result);
                            send(stream, data);
                        }
                    }
                    None => {
                        error!("No buffermap found");
                    }
                }
            }
            None => {
                error!("No stream found");
            }
        }
    }
    // format the data to be sent to the client

    // send the data to the clien
}

impl Task for Data {
    fn process(&mut self) {}
}

impl Task for Have {
    fn process(&mut self) {}
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_getpieces_process() {}
}
