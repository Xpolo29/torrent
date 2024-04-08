use crate::com::{connect, getfile_request, receive, send};
use crate::data::PeerConfig;
use crate::db::get_file;
use std;

/// takes a file and help know which peer to ask for what piece
fn get_peer_and_piece_indices(key: String) -> Vec<(PeerConfig, Vec<u32>)> {
    // fetch buffermap from all leechers/seeders that have it
    // push the rarest piece to the beginning of the vector
    todo!();
}

pub fn get_chunk_from_file(
    key: String,
    chunk_size: usize,
    chunk_index: usize,
) -> std::io::Result<Vec<u8>> {
    let meta_file = get_file(&key).ok_or(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "File not found",
    ))?;
    let file_path = &meta_file.file_name; // Assuming file_name is a field in meta_file
    get_chunk(file_path, chunk_size, chunk_index)
}

use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;

fn get_chunk(file_path: &str, chunk_size: usize, chunk_index: usize) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let start = chunk_size * chunk_index;
    file.seek(SeekFrom::Start(start as u64))?;
    let mut buffer = vec![0; chunk_size];
    let bytes_read = file.read(&mut buffer)?;
    buffer.truncate(bytes_read);
    Ok(buffer)
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
