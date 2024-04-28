//use crate::back::get_peer_and_piece_indices;
use crate::back::{get_chunks_from_file, get_wanted_piece_from_peer, FileAssembler};
use crate::com::{connect, receive, send};
use crate::data::PeerConfig;
use crate::db::{get_buffermap, get_file, get_peer_key, set_buffermap};
use crate::parser::parse_have_from_have;
use crate::tasks::{Data, DataWrite, Getpieces, Have, Interested, Peer, Task};
use hashbrown::HashMap;
use log::error;
/// `Getpieces` is a struct that implements the `Task` trait. It is used to send pieces of a file over a TCP stream.
///
/// # Process Method
/// The `process` method is responsible for sending the pieces of a file over a TCP stream.
///
/// It first checks if a stream is available. If not, it logs an error message.
/// If a stream is available, it retrieves the key and piece indices from the `Getpieces` struct.
/// It then uses the `get_chunks_from_file` function to retrieve the pieces of the file corresponding to the piece indices.
/// The pieces are then formatted into a string, with each piece represented as "index:piece".
/// A message is then constructed with the format "data key [index1:piece1 index2:piece2 ...]" and sent over the stream.
///
/// # Arguments
/// * `stream` - A mutable reference to an Option wrapping a TcpStream. This is the stream over which the data will be sent.
/// * `key` - A string representing the key of the file.
/// * `pieces` - A vector of u32s representing the indices of the pieces to be sent.
// write a data to TCP and update db
impl Task for Getpieces {
    // a file key and a list of index of pieces
    // send data key [index1:piece1 index2:piece2 ...]
    fn process(&mut self) {
        let stream = &mut self.stream;
        let chunk_size = 1024 * 8;
        match stream {
            Some(stream) => {
                // get key from getpieces
                let key = &self.key;
                // get the indexes of each piece
                let piece_indexes = &self.pieces;
                let data: HashMap<u32, Vec<u8>> =
                    get_chunks_from_file(key.clone(), chunk_size, piece_indexes.clone());
                // piece are already in the form of a byte sequence wrapped in a string
                let piece_array: Vec<String> = data
                    .iter()
                    .map(|(index, piece)| {
                        format!(
                            "{}:{}",
                            index,
                            piece
                                .iter()
                                .map(|x| x.to_string())
                                .collect::<Vec<String>>()
                                .join(" ")
                        )
                    })
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

/// `Data` is a struct that implements the `Task` trait. It is used to write received pieces of a file to the local file system.
///
/// # Process Method
/// The `process` method is responsible for writing the received pieces of a file to the local file system.
///
/// It first checks if a stream is available. If not, it logs an error message.
/// If a stream is available, it retrieves the key and pieces from the `Data` struct.
/// It then uses the `get_file` function to retrieve the file path for the key.
/// A `FileAssembler` is then created for the file path, and each piece is added to the `FileAssembler`.
/// After all pieces have been added, an "ok\n" message is sent over the stream.
///
/// # Arguments
/// * `stream` - A mutable reference to an Option wrapping a TcpStream. This is the stream over which the data was received.
/// * `key` - A string representing the key of the file.
/// * `pieces` - A HashMap where the keys are u32s representing the indices of the pieces and the values are the pieces themselves.
// send the data to the clien
// get data and write it to file
impl Task for Data {
    fn process(&mut self) {
        match &mut self.stream {
            Some(stream) => {
                let key = &self.key;
                let pieces = &self.pieces;
                let file_path = get_file(key).unwrap().file_name;
                let mut file_assembler = FileAssembler::new(&file_path, 1024 * 8).unwrap();
                for (index, piece) in pieces.iter() {
                    file_assembler.add_chunk(*index, piece.clone()).unwrap();
                }
                send(stream, "ok\n".to_string());
            }
            None => {
                error!("No stream found");
            }
        }
    }
}

/// `Have` is a struct that implements the `Task` trait. It is used to send a "have" message over a TCP stream.
///
/// # Process Method
/// The `process` method is responsible for sending a "have" message over a TCP stream.
/// It first checks if a stream is available. If not, it logs an error message.
/// If a stream is available, it retrieves the key from the `Have` struct and creates a `PeerConfig` from the current configuration.
/// It then uses the `get_buffermap` function to retrieve the buffer map for the key.
/// A message is then constructed with the format "have key buffermap" and sent over the stream.
///
/// # Arguments
/// * `stream` - A mutable reference to an Option wrapping a TcpStream. This is the stream over which the message will be sent.
/// * `key` - A string representing the key of the file.
// send a getpieces message to TCP
// recieve a key and a buffermap and sens his key and his buffermap
impl Task for Have {
    fn process(&mut self) {
        // create a peer_config from ip, and port taken by the stream
        let stream = &mut self.stream;
        match stream {
            Some(stream) => {
                let key = self.key.clone();
                let config = PeerConfig::from_config();
                let buffermap = get_buffermap(config, &key);
                let message = format!(
                    "have {} {}",
                    key,
                    String::from_utf8(buffermap.unwrap()).expect("Found invalid UTF-8")
                );
                send(stream, message);
            }
            None => {
                error!("No stream found");
            }
        }
        // get the index that current peer wants from the peer that sent the have message specificcly
    }
}

/// `Interested` is a struct that implements the `Task` trait. It is used to send a "have" message over a TCP stream.
///
/// # Process Method
/// The `process` method is responsible for sending a "have" message over a TCP stream.
/// It first checks if a stream is available. If not, it logs an error message.
/// If a stream is available, it retrieves the key from the `Interested` struct and the peer address from the stream.
/// A `PeerConfig` is then created from the peer address, and the `get_buffermap` function is used to retrieve the buffer map for the key.
/// If a buffer map is found, it is formatted into a string and a message is constructed with the format "data key buffermap" and sent over the stream.
/// If no buffer map is found, an error message is logged.
///
/// # Arguments
/// * `stream` - A mutable reference to an Option wrapping a TcpStream. This is the stream over which the message will be sent.
/// * `key` - A string representing the key of the file.
// send a have message to TCP
// have $Key $BufferMap
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
/// send a interested message to TCP
/// retrieve the buffermap update db
/// compute pieces to be taken relatvly to others and in function of the adressed peer
/// yield a task that send a getpiecce and recieve a data and write if (DataWrite)
impl Task for Peer {
    fn process(&mut self) {
        let adress = self.config.address.clone();
        let port = self.config.port;
        let file_key = &self.hash;
        let stream = &mut connect(port, &adress);
        match stream {
            Some(stream) => {
                let key = &self.hash;
                let message = format!("interested {}", key);
                send(stream, message);
                let response = receive(stream);
                // update db
                if let Some(have_struct) = parse_have_from_have(response) {
                    let buffermap = have_struct.buffermap;
                    let peer_key = get_peer_key(self.config.clone());
                    set_buffermap(file_key.clone(), peer_key.clone(), buffermap);
                    // get the pieces that the peer wants relativly to the other buffermap but included into the peers buffermap
                    let pieces = get_wanted_piece_from_peer(&peer_key, &file_key);
                }
                // return the DataWrite task
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
