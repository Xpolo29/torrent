use crate::com::{connect, getfile_request, receive, send};
use crate::data::PeerConfig;
use crate::db::get_file;
use crate::respons_handler::ExpectPeers;
use crate::respons_handler::ExpectedAnswer;
use crate::tasks::{Peers, Task};
use hashbrown::HashMap;
use log::error;
use md5::digest::block_buffer::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{Seek, SeekFrom, Write};
pub fn start_download(
    key: String,
    tracker_port: u16,
    tracker_adress: &str,
) -> Result<Vec<Box<dyn Task + Send>>, Error> {
    // extract the meta data from the file
    let meta_file = get_file(&key).unwrap();
    let chunk_size = meta_file.piece_size;
    // get the peers thare hold buffermap for the file
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
                    ExpectedAnswer::Peers(peers) => {
                        let mut tasks = Vec::new();
                        for peer in peers {
                            tasks.push(Box::new(Peers {
                                key: key.clone(),
                                peers: peer,
                            }));
                        }
                        tasks
                    }
                    _ => {
                        error!("couldn't retrieve peers from tracker");
                        Err(Box::new(Error))
                    }
                }
            }
            Err(valeur) => {
                error!("Tracker bad peers answer {}", valeur);
            }
        }
    }
}

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
fn get_chunk_from_file(key: String, chunk_size: u32, chunk_index: u32) -> std::io::Result<Vec<u8>> {
    let meta_file = get_file(&key).ok_or(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "File not found",
    ))?;
    let file_path = &meta_file.file_name; // Assuming file_name is a field in meta_file
    get_chunk(file_path, chunk_size, chunk_index)
}

fn get_chunk(file_path: &str, chunk_size: u32, chunk_index: u32) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let start = chunk_size * chunk_index;
    file.seek(SeekFrom::Start(start as u64))?;
    let mut buffer = vec![0; chunk_size as usize];
    let bytes_read = file.read(&mut buffer)?;
    buffer.truncate(bytes_read);
    Ok(buffer)
}
///
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
