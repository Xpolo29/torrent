//use crate::back::get_peer_and_piece_indices;
use crate::back::{
    get_chunks_from_file, get_wanted_piece_from_peer, is_stream_open, store_have_to_db,
    FileAssembler,
};
use crate::com::{connect, dataf, getpiecesf, havef, interestedf, receive, seedf, send};
use crate::data::{b64_enc, MetaFile, PeerConfig};
use crate::db::{
    get_buffermap, get_file, get_leeching_files, get_peer_key, get_seeding_files, log_db,
    set_buffermap, set_peer_to_file,
};
use crate::parser::{parse_have_from_have, parse_request};
use crate::respons_handler::{Answer, ExpectData, ExpectOk, ExpectedAnswer};
use crate::tasks::{
    Data, DataWrite, EmptyTask, Getpieces, Have, Interested, Peer, Task, ToBeProcessed,
};
use crate::threads::{handle_client, Pool};
use log::{debug, error, info, trace};
use rayon::prelude::*;
use std::cmp::min;
use std::fs::OpenOptions;
use std::io::{Error, ErrorKind, Seek, SeekFrom, Write};
use std::mem;
use std::net::{SocketAddr, TcpStream};
use std::sync::Mutex;

impl Task for EmptyTask {
    fn process(&mut self) {
        debug!("Processing empty task");
        let stream = &mut self.stream;
        match stream {
            Some(stream) => {
                let msg: String = String::from("EMPTY");
                send(stream, msg);
            }
            None => return,
        }
    }
}

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
    fn process(&mut self) {
        trace!("Processing getpiece task");

        if self.retry > 20 {
            return;
        }

        let stream = &mut self.stream;
        match stream {
            Some(stream) => {
                if !is_stream_open(stream) {
                    return;
                }
                let piece_indexes = &self.pieces;
                if self.retry == 0 && piece_indexes.len() > 0 {
                    // get key from getpieces
                    let key = &self.key;
                    // get the indexes of each piece

                    trace!("Begin to read theses chunk {:?}", piece_indexes);
                    let data: Vec<(usize, Vec<u8>)> =
                        get_chunks_from_file(key.to_string(), self.chunk_size, piece_indexes);

                    let pieces: Vec<String> = data
                        .par_iter()
                        .map(|piece| {
                            let cur_index: usize = piece.0;
                            let cur_data: Vec<u8> = piece.1.clone();
                            let cur_data_str: String = b64_enc(cur_data);
                            format!("{}:{}", cur_index, cur_data_str)
                        })
                        .collect();

                    let message: String = dataf(key, pieces);

                    send(stream, message);
                }

                // add a new task
                let next_pieces: String = receive(&mut self.stream.as_mut().unwrap(), 250);

                let next: Box<dyn Task + Send>;
                if next_pieces.len() == 0 {
                    next = Box::new(Getpieces {
                        key: self.key.clone(),
                        chunk_size: self.chunk_size,
                        pieces: Vec::new(),
                        stream: self.stream.take(),
                        pool: self.pool.clone(),
                        retry: self.retry + 1,
                    });
                } else {
                    next = parse_request(next_pieces, self.stream.take(), self.pool.clone());
                }

                self.pool.add_task(next);
            }
            None => {
                //error!("No stream found");
                debug!("No stream found for getpiece");
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
        trace!("Processing data task");
        return;
        // deprecated ? for now DataWrite is used
        /*
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
        */
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
impl Task for Have {
    fn process(&mut self) {
        trace!("Processing have task");

        // update db with new buffermap
        let have: Have = self.clone();
        let stream_clone: TcpStream;
        match self.stream.as_ref().unwrap().try_clone() {
            Ok(v) => stream_clone = v,
            Err(e) => {
                error!("Could not add have to db {}", e);
                return;
            }
        }
        let addr: SocketAddr = stream_clone.peer_addr().unwrap();
        let address: String = addr.ip().to_string();
        let port: u16 = addr.port();

        let config: PeerConfig = PeerConfig { address, port };

        store_have_to_db(config, have);

        // answer with own buffermap
        // create a peer_config from ip, and port taken by the stream
        let stream = &mut self.stream;
        match stream {
            Some(stream) => {
                let key = self.key.clone();
                let config = PeerConfig::new();
                let buffermap_option: Option<Vec<u8>> = get_buffermap(config, &key);
                let buffermap: Vec<u8>;

                match buffermap_option {
                    Some(arr) => buffermap = arr,
                    None => {
                        let len: usize = self.buffermap.len();
                        // create empty buffermap
                        buffermap = vec![0; len];
                    }
                }
                let message: String = havef(key, buffermap);

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
        trace!("Processing interested task");
        let stream = &mut self.stream;
        match stream {
            Some(stream) => {
                // receive interested key
                let key = &self.key;
                let peerconfig = PeerConfig::new();
                // get buffermap from the database
                let buffermap_option: Option<Vec<u8>> = get_buffermap(peerconfig, key);
                let buffermap: Vec<u8>;

                match buffermap_option {
                    Some(arr) => {
                        buffermap = arr;
                    }
                    None => {
                        error!("No buffermap found");
                        return;
                    }
                }

                // convert [0, 0, 1, 0] to 0010
                let buffermap = buffermap
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join("");

                let message = format!("have {} {}", key, buffermap);
                send(stream, message);
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
        trace!("Processing peer task");
        let adress = self.config.address.clone();
        let port = self.config.port;
        let file_key = &self.hash;
        debug!("Trying to connect to {}", self.config.address.clone());
        let stream = &mut connect(port, &adress);
        match stream {
            Some(stream) => {
                let key: String = self.hash.clone();

                // send interested to download
                let message = interestedf(key);
                debug!("Sending {} to {}", message, self.config.address.clone());
                send(stream, message);
                let response: String = receive(stream, 3000);
                let cut: String = response.chars().take(128).collect::<String>();
                debug!("Received {} from {}", cut, self.config.address.clone());
                // update db
                // retrieve data
                if let Some(have_struct) = parse_have_from_have(response) {
                    // instead of adding the task to the pool, just update db
                    let buffermap = have_struct.buffermap;
                    let peer_key = get_peer_key(self.config.clone());
                    set_buffermap(file_key.clone(), peer_key.clone(), buffermap);
                    // get the pieces that the peer wants relativly to the other buffermap but included into the peers buffermap
                    //let pieces = get_wanted_piece_from_peer(&peer_key, &file_key);
                }
                let chunk_size: usize;
                let file_option: Option<MetaFile> = get_file(&self.hash);
                match file_option {
                    Some(file) => chunk_size = file.piece_size,
                    None => chunk_size = 1024,
                }

                let nb_pieces: usize = self.length_tcp / chunk_size;

                // create the DataWrite task
                let peer: PeerConfig = self.config.clone();
                let file_key: String = self.hash.clone();
                let pool: Pool = self.pool.clone();
                let stream = None;

                let ret: DataWrite = DataWrite {
                    peer,
                    file_key,
                    nb_pieces,
                    pool,
                    stream,
                };

                // Arbitrary number of task,
                for _ in 0..3 {
                    let ret_clone = ret.clone();
                    self.pool.add_task(Box::new(ret_clone));
                }

                // Add DataWrite task to queue
                //self.pool.add_task(Box::new(ret));
            }
            None => {
                error!("No stream found");
            }
        }
    }
}

impl Task for DataWrite {
    fn process(&mut self) {
        trace!("Processing DataWrite task");
        let peer: PeerConfig = self.peer.clone();
        let hash: String = self.file_key.clone();
        let pieces: Vec<usize> =
            get_wanted_piece_from_peer(&get_peer_key(peer), &hash, self.nb_pieces);

        // if there is nothing left to download, exit
        if pieces.len() == 0 {
            return;
        }

        match self.stream.as_ref() {
            Some(_) => (),
            None => {
                trace!("Stream is closed, opening new one");
                self.stream = connect(self.peer.port, &self.peer.address)
            }
        }

        let msg = getpiecesf(self.file_key.clone(), pieces.clone());

        let answer: String;
        match self.stream.as_mut() {
            Some(mut stream) => {
                send(&mut stream, msg);
                answer = receive(&mut stream, 3000)
            }
            None => {
                error!("Downloading stream closed prematurarily");
                return;
            }
        }

        // init future buffermap
        //trace!("peer : {:?}, hash : {}", PeerConfig::new(), &self.file_key);
        let buffmap_option = get_buffermap(PeerConfig::new(), &self.file_key.clone());
        //let new_buffermap: Mutex<Vec<u8>>;
        let mut new_buffermap: Vec<u8>;
        match buffmap_option {
            Some(arr) => new_buffermap = arr,
            None => {
                error!("Got piece of unknown file");
                return;
            }
        }

        //let received_pieces: Mutex<Vec<usize>> = Mutex::new(pieces.clone());
        let mut received_pieces: Vec<usize> = pieces.clone();

        // to prevent multiple thread to ask for the same pieces
        set_buffermap(
            self.file_key.clone(),
            get_peer_key(PeerConfig::new()),
            new_buffermap.clone(),
        );

        // Parse answer
        match ExpectData.check_answer(&answer) {
            Ok(answer) => {
                let answer: Answer = ExpectData.retrieve_data(answer);
                match answer {
                    Answer::Data(data) => {
                        //data is Vec<(usize, String)>

                        // open file only once
                        let writer: MetaFile;
                        match get_file(&self.file_key.clone()) {
                            Some(value) => writer = value,
                            None => {
                                error!("Could not find file {} metadata in db", self.file_key);
                                return;
                            }
                        }
                        let filename: String = writer.file_name.clone();
                        let mut file = OpenOptions::new()
                            .write(true)
                            .create(true)
                            .open(&filename)
                            .expect("Unable to open file");

                        for entry in data {
                            let index: usize = entry.0;
                            let chunk: Vec<u8> = entry.clone().1;
                            //received_pieces.retain(|&x| x != index);
                            received_pieces = received_pieces
                                .into_iter()
                                .filter(|&x| x != index)
                                .collect();

                            // Calculate the offset based on the index and piece_size
                            let offset = index * writer.piece_size;

                            // Seek to the desired position in the file
                            file.seek(SeekFrom::Start(offset as u64))
                                .expect("Unable to seek");

                            // Write the chunk to the file
                            let ok = file.write_all(&chunk);
                            match ok {
                                Ok(_) => {}
                                Err(e) => {
                                    error!("Error writing to disk : {}", e);
                                    return;
                                }
                            }
                        }
                    }
                    _ => error!("couldn't retrieve data from peer"),
                }
            }
            Err(e) => {
                if let Some(io_err) = e.downcast_ref::<Error>() {
                    if io_err.kind() == ErrorKind::InvalidInput {
                    } else {
                        error!("Wrong answer from getpiece {}", e);
                        return;
                    }
                } else {
                    error!("Wrong answer from getpiece {}", e);
                    return;
                }
            }
        }

        // check is there is some control to do
        {
            let not_received_pieces = received_pieces.clone();
            for not_received in not_received_pieces {
                new_buffermap[not_received] = 0;
            }
        }

        //let new_buffermap: Vec<u8> = new_buffermap.lock().unwrap().clone();
        // update db if missing some pieces
        if received_pieces.len() > 0 {
            set_buffermap(
                self.file_key.clone(),
                get_peer_key(PeerConfig::new()),
                new_buffermap,
            );
        }

        // re adding oneself to continue downloading
        let peer: PeerConfig = self.peer.clone();
        let file_key: String = self.file_key.clone();
        let nb_pieces: usize = self.nb_pieces;
        let pool: Pool = self.pool.clone();

        let next: DataWrite = DataWrite {
            peer,
            file_key,
            nb_pieces,
            pool,
            stream: self.stream.take(),
        };

        self.pool.add_task(Box::new(next));
    }
}

// incoming connection task waiting to be processed
impl Task for ToBeProcessed {
    fn process(&mut self) {
        trace!("Processing ToBeProcessed task");
        let stream = self.stream.try_clone().unwrap();
        handle_client(self.pool.clone(), stream);
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
            chunk_size: 1024,
            pieces: vec![0, 1, 2],
            stream: Some(stream),
            pool: Pool::new(0),
        };

        // Call the process method
        getpieces.process();

        // Check that the function correctly modified the database
        let result = get_buffermap(peer_config, &file_key);
        assert_eq!(result, Some(buffermap));
    }
}
