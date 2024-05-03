use ini::Ini;
use md5::{Digest, Md5};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
#[derive(Debug, Clone)]
pub struct MetaFile {
    pub file_name: String,
    pub length: u64,
    pub piece_size: u64,
    pub hash: String,
}

impl MetaFile {
    pub fn new(file_name: String) -> Self {
        let path = Path::new(&file_name);
        let length = path.metadata().unwrap().len();
        MetaFile {
            hash: get_file_key(&file_name),
            file_name,
            length,
            piece_size: 1024,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PeerConfig {
    pub address: String,
    pub port: u16,
}

impl PeerConfig {
    pub fn new(address: String, port: u16) -> Self {
        PeerConfig { address, port }
    }
}

pub struct TrackerConfig {
    pub address: String,
    pub port: u16,
}

impl TrackerConfig {
    pub fn new() -> Self {
        let conf = Ini::load_from_file("config.ini").unwrap();
        let tracker_section = conf.section(Some("Tracker")).unwrap();
        let peer_section = conf.section(Some("Peer")).unwrap();

        let tracker_address = tracker_section.get("tracker-address").unwrap().to_string();
        let tracker_port = tracker_section
            .get("tracker-port")
            .unwrap()
            .parse::<u16>()
            .unwrap();

        let _peer_address = peer_section.get("peer-address").unwrap().to_string();
        let _peer_port = peer_section
            .get("peer-port")
            .unwrap()
            .parse::<u16>()
            .unwrap();

        TrackerConfig {
            address: tracker_address,
            port: tracker_port,
        }
    }
}

impl PeerConfig {
    pub fn from_config() -> Self {
        let conf = Ini::load_from_file("config.ini").unwrap();
        let peer_section = conf.section(Some("Peer")).unwrap();
        let peer_address = peer_section.get("peer-address").unwrap().to_string();
        let peer_port = peer_section
            .get("peer-port")
            .unwrap()
            .parse::<u16>()
            .unwrap();
        PeerConfig {
            address: peer_address,
            port: peer_port,
        }
    }
}

/// Computes the MD5 hash of a file.
///
/// This function takes a file path as a string.
/// It opens the file, reads its content, and computes the MD5 hash of the content.
/// It then returns the hash as a string.
///
/// # Arguments
/// * `path` - A string slice representing the file path.
///
/// # Returns
/// * `String` - The MD5 hash of the file content.
pub fn get_file_key(path: &str) -> String {
    let path = Path::new(path);
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut hasher = Md5::new();
    std::io::copy(&mut reader, &mut hasher).unwrap();
    let result = hasher.finalize();
    format!("{:x}", result)
}

/// Computes the buffer size for a file.
///
/// # Arguments
/// * `file` - A reference to a MetaFile struct.
///
/// # Returns
/// * `u64` - The buffer size for the file.
pub fn get_buffer_size(file: &MetaFile) -> u64 {
    file.length / file.piece_size + 1
}

/// Gets the hash of a file.
///
/// # Arguments
/// * `file` - A reference to a MetaFile struct.
///
/// # Returns
/// * `String` - The hash of the file.
pub fn get_file_hash(file: &MetaFile) -> String {
    file.hash.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peer_config_from_config() {
        // Set up the test
        let peer_config = PeerConfig::from_config();
        assert_eq!(peer_config.address, "127.0.0.1");
        assert_eq!(peer_config.port, 54321);
    }
}
