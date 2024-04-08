use crate::com::{connect, getfile_request, receive, send};
use crate::data::PeerConfig;
use crate::db::get_file;
use hashbrown::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{Seek, SeekFrom, Write};

/// takes a file and help know which peer to ask for what piece
fn get_peer_and_piece_indices(key: String) -> Vec<(PeerConfig, Vec<u32>)> {
    // fetch buffermap from all leechers/seeders that have it
    // push the rarest piece to the beginning of the vector
    todo!();
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
