use crate::data::{TrackerConfig,PeerConfig,MetaFile,};
use log::{info, warn};
/// Takes a list of seeded and leeched files with medata data and returns the right message to be sent
pub fn seed(seeded: Vec<MetaFile>, peer_port: String, leeched: String) -> String{
/*
    into_iter() : transform the vector into an iterator
    map() : apply a function to each element of the iterator
    collect() : transform the iterator into a vector
    join() : concatenate the elements of the vector into a single string
     */
    let seeded_string: Vec<String> = seeded
        .into_iter()
        .map(|file| {
            format!(
                "{} {} {} {}",
                file.file_name, file.length, file.piece_size, file.hash
            )
        })
        .collect();

    let msg = format!(
        "announce listen {} seed [{}] leech [{}]\r\n",
        peer_port,
        seeded_string.join(" "),
        leeched
    ); 
    msg
}

pub fn send(message: String, port: u16, adress: String) {
info!("Sending to {}:{} < {}", adress, port, message);
}









#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seed() {
        // Arrange
        let seeded = vec![
            MetaFile {
                file_name: String::from("file1.txt"),
                length: 100,
                piece_size: 10,
                hash: String::from("hash1"),
            },
            MetaFile {
                file_name: String::from("file2.txt"),
                length: 200,
                piece_size: 20,
                hash: String::from("hash2"),
            },
        ];
        
    }
}
