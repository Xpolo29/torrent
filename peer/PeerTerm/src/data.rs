use base64::{engine::general_purpose, Engine as _};
use ini::Ini;
use lazy_static::lazy_static;
use log::{debug, info};
use md5::{Digest, Md5};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Mutex;
#[derive(Debug, Clone)]
pub struct MetaFile {
    pub file_name: String,
    pub length: usize,
    pub piece_size: usize,
    pub hash: String,
}

impl MetaFile {
    pub fn new(file_name: String) -> Self {
        let path = Path::new(&file_name);
        let length = path.metadata().unwrap().len() as usize;
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

#[derive(Clone, Debug)]
pub struct TrackerConfig {
    pub address: String,
    pub port: u16,
}

impl TrackerConfig {
    pub fn new() -> Self {
        let tracker_address = {
            let lock = TRACKER_ADDRESS.lock().unwrap();
            if let Some(address) = lock.clone() {
                address
            } else {
                drop(lock); // Release the lock before reading the config file
                let config_path = {
                    let lock = CONFIG_PATH.lock().unwrap();
                    lock.clone().unwrap_or_else(|| "config.ini".to_string())
                };
                let conf = Ini::load_from_file(&config_path).unwrap();
                let tracker_section = conf.section(Some("Tracker")).unwrap();
                tracker_section.get("tracker-address").unwrap().to_string()
            }
        };
        let tracker_port = {
            let lock = TRACKER_PORT.lock().unwrap();
            if let Some(port) = lock.clone() {
                port
            } else {
                drop(lock); // Release the lock before reading the config file
                let config_path = {
                    let lock = CONFIG_PATH.lock().unwrap();
                    lock.clone().unwrap_or_else(|| "config.ini".to_string())
                };
                let conf = Ini::load_from_file(&config_path).unwrap();
                let tracker_section = conf.section(Some("Tracker")).unwrap();
                tracker_section
                    .get("tracker-port")
                    .unwrap()
                    .parse::<u16>()
                    .unwrap()
            }
        };

        debug!(
            "CONSTRUCTEUR: tracker_adress: {} tracker_port: {} ",
            tracker_address, tracker_port
        );
        TrackerConfig {
            address: tracker_address,
            port: tracker_port,
        }
    }
}

lazy_static! {
    static ref CONFIG_PATH: Mutex<Option<String>> = Mutex::new(None);
    static ref TRACKER_PORT: Mutex<Option<u16>> = Mutex::new(None);
    static ref TRACKER_ADDRESS: Mutex<Option<String>> = Mutex::new(None);
    static ref PEER_PORT: Mutex<Option<u16>> = Mutex::new(None);
}

pub fn set_config_path(path: String) {
    let mut config_path = CONFIG_PATH.lock().unwrap();
    *config_path = Some(path);
}
pub fn set_tracker_port(port: u16) {
    let mut tracker_port = TRACKER_PORT.lock().unwrap();
    *tracker_port = Some(port);
}
pub fn set_tracker_address(address: String) {
    let mut tracker_address = TRACKER_ADDRESS.lock().unwrap();
    *tracker_address = Some(address);
}
pub fn set_peer_port(port: u16) {
    let mut peer_port = PEER_PORT.lock().unwrap();
    *peer_port = Some(port);
}

impl PeerConfig {
    pub fn new() -> Self {
        let config_path = {
            let lock = CONFIG_PATH.lock().unwrap();
            lock.clone().unwrap_or_else(|| "config.ini".to_string())
        };
        let conf = Ini::load_from_file(&config_path).unwrap();
        let peer_section = conf.section(Some("Peer")).unwrap();

        let peer_address = peer_section.get("peer-address").unwrap().to_string();

        let peer_port = {
            let lock = PEER_PORT.lock().unwrap();
            if let Some(port) = lock.clone() {
                port
            } else {
                drop(lock); // Release the lock before reading the config file
                peer_section
                    .get("peer-port")
                    .unwrap()
                    .parse::<u16>()
                    .unwrap()
            }
        };
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
/// * `usize` - The buffer size for the file.
pub fn get_buffer_size(file: &MetaFile) -> usize {
    file.length / file.piece_size + 1
}

/// Return the base64 encoded string from bytes array
pub fn b64_enc(data: Vec<u8>) -> String {
    general_purpose::STANDARD.encode(data)
}

/// Return the decoded string from bytes array
pub fn b64_dec(base64_string: String) -> Vec<u8> {
    general_purpose::STANDARD.decode(base64_string).unwrap()
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
    fn test_peer_config_new() {
        // Set up the test
        let peer_config = PeerConfig::new();
        assert_eq!(peer_config.address, "0.0.0.0");
        assert_eq!(peer_config.port, 54321);
    }
}
