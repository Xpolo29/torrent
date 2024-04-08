//use crate::back::get_peer_and_piece_indices;
use crate::back::{get_chunk_from_file, get_wanted_piece_from_peer};
use crate::com::send;
use crate::data::PeerConfig;
use crate::db::get_buffermap;
use crate::tasks::{Data, Getpieces, Have, Interested, Task};
use log::{error, trace};
/// write a data to TCP
impl Task for Getpieces {
    /// a file key and a list of index of pieces
    /// send data key [index1:piece1 index2:piece2 ...]
    fn process(&mut self) {
        let stream = &mut self.stream;
        let chunk_size = 1024 * 8;
        match stream {
            Some(stream) => {
                // get key from getpieces
                let key = &self.key;
                // get the indexes of each piece
                let piece_indexes = &self.pieces;
                let data: Vec<(u32, String)> = todo!("chunk_size");
                // piece are already in the form of a byte sequence wrapped in a string
                let piece_array: Vec<String> = data
                    .iter()
                    .map(|(index, piece)| format!("{}:{}", index, piece))
                    .collect();
                let message = format!("data {} [{}]", key, piece_array.join(" "));
                send(stream, message);
            }
            None => {
                error!("No stream found");
            }
        }
    }
}
// format the data to be sent to the client

// send the data to the clien

/// get data and write it to file
impl Task for Data {
    fn process(&mut self) {}
}

/// send a getpieces message to TCP
impl Task for Have {
    fn process(&mut self) {
        // create a peer_config from ip, and port taken by the stream
        let stream = &mut self.stream;
        match stream {
            Some(stream) => {
                let key = &self.key;
                let peer_addr = stream.peer_addr().unwrap();
                let port = peer_addr.port();
                let ip = peer_addr.ip();
                let peerconfig = PeerConfig {
                    address: ip.to_string(),
                    port,
                };
                let wanted_indexes: Vec<u32> = get_wanted_piece_from_peer(peerconfig);
                let wanted_indexes_str: Vec<String> =
                    wanted_indexes.iter().map(|i| i.to_string()).collect();
                let message = format!("getpieces {} [{}]", key, wanted_indexes_str.join(" "));
                send(stream, message);
            }
            None => {
                error!("No stream found");
            }
        }
        // get the index that current peer wants from the peer that sent the have message specificcly
    }
}
// send a have message to TCP
impl Task for Interested {
    fn process(&mut self) {
        let stream = &mut self.stream;
        match stream {
            Some(stream) => {
                // receive interested key
                let key = &self.key;
                let peer_addr = stream.peer_addr().unwrap();
                // create peerconfig to pass to get_buffermap
                let port = peer_addr.port();
                let ip = peer_addr.ip();
                let peerconfig = PeerConfig {
                    address: ip.to_string(),
                    port,
                };
                // get buffermap from the database
                let buffermap = get_buffermap(peerconfig, key);

                match buffermap {
                    Some(buffermap) => {
                        // buffermap = [0,1,0,1,0]
                        let buffermap = buffermap
                            .iter()
                            .map(|x| x.to_string())
                            .collect::<Vec<String>>()
                            .join(" ");
                        let message = format!("data {} {}", key, buffermap);
                        send(stream, message);
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{get_buffermap, get_peer_key, set_buffermap};
    use std::net::{TcpListener, TcpStream};

    #[test]
    fn test_getpieces_process() {
        // Set up the database in a known state
        let file_key = "test_file_key".to_string();
        let peer_config = PeerConfig {
            address: "127.0.0.1".to_string(),
            port: 8080,
        };
        let peer_key = get_peer_key(peer_config.clone());
        let buffermap = vec![0u8, 1, 2, 3, 4];
        set_buffermap(file_key.clone(), peer_key.clone(), buffermap.clone());

        // Create a TcpStream for testing
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let stream = TcpStream::connect(("127.0.0.1", port)).unwrap();

        // Create a Getpieces instance
        let mut getpieces = Getpieces {
            key: file_key.clone(),
            pieces: vec![0, 1, 2],
            stream: Some(stream),
        };

        // Call the process method
        getpieces.process();

        // Check that the function correctly modified the database
        let result = get_buffermap(peer_config, &file_key);
        assert_eq!(result, Some(buffermap));
    }
}
