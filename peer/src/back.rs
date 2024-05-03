use crate::com::{connect, getfile_request, receive, send};
use crate::data::PeerConfig;
use crate::db::{get_file, get_peer_from_file, get_buffermap, get_peer_key};
use crate::respons_handler::{Answer, ExpectPeers, ExpectedAnswer};
use crate::tasks::{Peer};
use log::{error, trace};
use md5::digest::block_buffer::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{Seek, SeekFrom, Write};
use crate::threads::Pool;
use std::cmp::min;

/// Starts the download process for a file.
///
/// This function connects to a tracker, sends a request for the file, and receives a response.
/// It then checks the response and retrieves the peers that hold the file.
/// For each peer, it creates a new task and adds it to a vector of tasks.
///
/// # Arguments
/// * `key` - A string that holds the key of the file to be downloaded.
/// * `tracker_port` - A u16 that represents the port of the tracker.
/// * `tracker_adress` - A string slice that holds the address of the tracker.
///
/// # Returns
/// * `Result<Vec<Box<Peer>>, Error>` - A Result which is either:
///     * `Ok(Vec<Box<Peer>>)` - A vector of boxed tasks if the operation is successful.
///     * `Err(Error)` - An error if the operation fails.
pub fn start_download(
    key: String,
    tracker_port: u16,
    tracker_adress: &str,
    pool: Pool,
) -> Result<Vec<Box<Peer>>, Error> {
    // extract the meta data from the file
    // let meta_file = get_file(&key).unwrap();
    // let chunk_size = meta_file.piece_size;
    // get the peers thare hold buffermap for the file
    if let Some(mut stream) = connect(tracker_port, &tracker_adress) {
        let getfile_message = getfile_request(key);

        // send getfile
        send(&mut stream, getfile_message);

        // get answer
        let response = receive(&mut stream);
        // check if answer is valid
        match ExpectPeers.check_answer(&response) {
            Ok(valeur) => {
                trace!("{}", valeur);
                // peers that hold each buffermap
                // Here retrieve peers from correct answer
                let peers: Answer = ExpectPeers.retrieve_data(valeur);
                // now we should ask each peer for their buffermap that is a task
                match peers {
                    Answer::Peers(peers) => {
                        let mut tasks = Vec::new();
                        for mut peer in peers {
                            // retrieve data init pool with an empty one
                            // so we need to overwrite it
                            let pool: Pool = pool.clone();
                            peer.pool = pool; 
                            tasks.push(Box::new(peer));
                        }
                        Ok(tasks)
                    }
                    _ => {
                        error!("couldn't retrieve peers from tracker");
                        Err(Error)
                    }
                }
            }
            Err(valeur) => {
                error!("{}", valeur);
                Err(Error)
            }
        }
    } else {
        Err(Error)
    }
}

/// Retrieves the specified chunks from a file.
///
/// This function iterates over a vector of chunk indices, retrieves each chunk from the file,
/// and stores it in a HashMap where the key is the chunk index and the value is the chunk data.
///
/// # Arguments
/// * `key` - A string that holds the key of the file.
/// * `chunk_size` - A u32 that represents the size of each chunk.
/// * `chunk_array` - A vector of u32s that represents the indices of the chunks to be retrieved.
///
/// # Returns
/// * `HashMap<u32, Vec<u8>>` - A HashMap where the key is the chunk index and the value is the chunk data.
pub fn get_chunks_from_file(
    key: String,
    chunk_size: usize,
    chunk_indexes: Vec<usize>,
) -> Vec<(usize, Vec<u8>)> {
    let mut chunks = Vec::new();
    for chunk_index in chunk_indexes {
        let chunk = get_chunk_from_file(key.clone(), chunk_size, chunk_index).unwrap();
        chunks.push((chunk_index, chunk));
    }
    chunks
}

/// Retrieves a specific chunk from a file.
///
/// This function gets the metadata of the file, constructs the file path, and retrieves the specified chunk.
///
/// # Arguments
/// * `key` - A string that holds the key of the file.
/// * `chunk_size` - A u32 that represents the size of each chunk.
/// * `chunk_index` - A u32 that represents the index of the chunk to be retrieved.
///
/// # Returns
/// * `std::io::Result<Vec<u8>>` - A Result which is either:
///     * `Ok(Vec<u8>)` - A vector of bytes representing the chunk if the operation is successful.
///     * `Err(std::io::Error)` - An error if the operation fails.
fn get_chunk_from_file(key: String, chunk_size: usize, chunk_index: usize) -> std::io::Result<Vec<u8>> {
    let meta_file = get_file(&key).ok_or(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "File not found",
    ))?;
    let file_path = &meta_file.file_name; // Assuming file_name is a field in meta_file
    get_chunk(file_path, chunk_size, chunk_index)
}

/// Retrieves a specific chunk from a file.
///
/// This function opens the file, seeks to the start of the specified chunk, reads the chunk into a buffer, and returns the buffer.
///
/// # Arguments
/// * `file_path` - A string slice that holds the path of the file.
/// * `chunk_size` - A u32 that represents the size of each chunk.
/// * `chunk_index` - A u32 that represents the index of the chunk to be retrieved.
///
/// # Returns
/// * `std::io::Result<Vec<u8>>` - A Result which is either:
///     * `Ok(Vec<u8>)` - A vector of bytes representing the chunk if the operation is successful.
///     * `Err(std::io::Error)` - An error if the operation fails.
fn get_chunk(file_path: &str, chunk_size: usize, chunk_index: usize) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let start = chunk_size * chunk_index;
    file.seek(SeekFrom::Start(start as u64))?;
    let mut buffer = vec![0; chunk_size as usize];
    let bytes_read = file.read(&mut buffer)?;
    buffer.truncate(bytes_read);
    Ok(buffer)
}


pub fn get_wanted_piece_from_peer(peer_key: &str, file_key: &str) -> Vec<usize> {


    let file_key_clone = String::from(file_key.clone());
    let peers: Vec<PeerConfig> = get_peer_from_file(file_key_clone);

    let mut buffmaps: Vec<Vec<u8>> = Vec::new();

    // cycle through all peers to get their buffmap
    for peer in peers {
        let file_key_clone = file_key.clone();
        let peer_clone = peer.clone();
        let option: Option<Vec<u8>> = get_buffermap(peer, file_key_clone);
        match option {
            Some(buffmap) => buffmaps.push(buffmap),
            None => {},
        }
    }

    // now we need to calculate the rarest parts scores
    let buffmaps_clone = buffmaps.clone();
    let main_buffmap: Vec<u8>;
    match get_buffermap(PeerConfig::from_config(), file_key.clone()){
        Some(arr) => main_buffmap = arr,
        None => main_buffmap = vec![0 as u8; buffmaps[0].len()],
    }
    let len: usize = main_buffmap.len();
    let mut scores: Vec<usize> = vec![0; len];
    let mut remaining: u32 = 0;

    trace!("main_buffmap : {:?}, len {}", main_buffmap, len);

    for i in 0..len {
        if main_buffmap[i] == 0{
            remaining += 1;
            for buffmap in &buffmaps_clone {
                if buffmap[i] == 1{
                    scores[i] += 1;
                }
            }
        }
    };

    // now we have a score per index
    // we just need to the get lowest scores

    // TODO How many pieces to get? hardcoded 5 for now

    let nb_pieces = min(5, remaining);
    trace!("score map {:?} of len {}, remaining {}", scores, scores.len(), remaining);
    let mut ret: Vec<usize> = Vec::new();

    for _ in 0..nb_pieces {
        let mut minimal: usize = 999;
        let mut index: usize = 0;
       
        for i in 0..scores.len() {
            let b1 = scores[i] < minimal;
            let b2 = ret.iter().any(|&j| j == i);
            if b1 && !b2 {
                minimal = scores[i];
                index = i + 1;
            }
        }

        if index == 0 {error!("Piece selection error"); continue}
        ret.push(index - 1);
    }

    trace!("Rarest parts are {:?}", ret);

    ret
}

pub struct FileAssembler {
    file: std::fs::File,
    chunk_size: u32,
}
impl FileAssembler {
    pub fn new(file_path: &str, chunk_size: u32) -> std::io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)?;

        Ok(Self { file, chunk_size })
    }

    pub fn add_chunk(&mut self, index: u32, data: Vec<u8>) -> std::io::Result<()> {
        self.file
            .seek(SeekFrom::Start((index * self.chunk_size) as u64))?;
        self.file.write_all(&data)?;
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_get_chunk() -> std::io::Result<()> {
        // Create a test file with known content
        let file_path = "test_file.txt";
        let mut file = File::create(file_path)?;
        file.write_all(b"Hello, world!")?;

        // Read the first chunk from the file
        let chunk = get_chunk(file_path, 5, 0)?;

        // Check that the chunk content is correct
        assert_eq!(chunk, b"Hello");

        Ok(())
    }
}
