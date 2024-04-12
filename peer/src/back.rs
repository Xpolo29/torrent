use crate::com::{connect, getfile_request, receive, send};
use crate::data::PeerConfig;
use crate::db::get_file;
use crate::respons_handler::{Answer, ExpectPeers, ExpectedAnswer};
use crate::tasks::{Peer, Task};
use hashbrown::HashMap;
use log::error;
use md5::digest::block_buffer::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{Seek, SeekFrom, Write};



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
/// * `Result<Vec<Box<dyn Task + Send>>, Error>` - A Result which is either:
///     * `Ok(Vec<Box<dyn Task + Send>>)` - A vector of boxed tasks if the operation is successful.
///     * `Err(Error)` - An error if the operation fails.
pub fn start_download(
    key: String,
    tracker_port: u16,
    tracker_adress: &str,
) -> Result<Vec<Box<dyn Task + Send>>, Error> {
    // extract the meta data from the file
    let meta_file = get_file(&key).unwrap();
    let chunk_size = meta_file.piece_size;
    // get the peers that hold buffermap for the file
    if let Some(mut stream) = connect(tracker_port, &tracker_adress) {
        let getfile_message = getfile_request(key);
        send(&mut stream, getfile_message);
        let response = receive(&mut stream);
        match ExpectPeers.check_answer(&response) {
            Ok(valeur) => {
                // peers that hold each buffermap
                let peers = ExpectPeers.retrieve_data(response);
                // now we should ask each peer for their buffermap that is a task
                // create the peer task
                match peers {
                    Answer::Peers(peers) => {
                        let mut tasks = Vec::new();
                        for peer in peers {
                            tasks.push(Box::new(Peer {
                                hash: key.clone(),
                                config: peer.config,
                            }) as Box<dyn Task + Send>);
                        }
                        Ok(tasks)
                    }
                    _ => {
                        error!("Couldn't retrieve peers from tracker");
                        Err(Error)
                    }
                }
            }
            Err(valeur) => {
                error!("Tracker bad peers answer {}", valeur);
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
    chunk_size: u32,
    chunk_array: Vec<u32>,
) -> HashMap<u32, Vec<u8>> {
    let mut chunks = HashMap::new();
    for chunk_index in chunk_array {
        let chunk = get_chunk_from_file(key.clone(), chunk_size, chunk_index).unwrap();
        chunks.insert(chunk_index, chunk);
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
fn get_chunk_from_file(key: String, chunk_size: u32, chunk_index: u32) -> std::io::Result<Vec<u8>> {
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
fn get_chunk(file_path: &str, chunk_size: u32, chunk_index: u32) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let start = chunk_size * chunk_index;
    file.seek(SeekFrom::Start(start as u64))?;
    let mut buffer = vec![0; chunk_size as usize];
    let bytes_read = file.read(&mut buffer)?;
    buffer.truncate(bytes_read);
    Ok(buffer)
}


/// Not implemented yet
pub fn get_wanted_piece_from_peer(peer: PeerConfig) -> Vec<u32> {
    todo!();
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
