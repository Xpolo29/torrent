use crate::data::MetaFile;
use crate::data::PeerConfig;
use hashbrown::HashMap;
use lazy_static::lazy_static;
use log::{debug, error, info};
use std::sync::Mutex;
// use log{info};

// struct Data {
//     pub file: MetaFile,
//     pub peer: PeerConfig,
//     pub buffermap: Vec<u8>,
// }
//
// static db: Vec<Data> = Vec::new();
//
lazy_static! {
    static ref PEERSDB: Mutex<HashMap<String, PeerConfig>> = Mutex::new(HashMap::new());
    static ref FILEDB: Mutex<HashMap<String, MetaFile>> = Mutex::new(HashMap::new());
    static ref BUFFERMAPDB: Mutex<HashMap<String, HashMap<String, Vec<u8>>>> =
        Mutex::new(HashMap::new());
}

fn get_peer_key(peer: PeerConfig) -> String {
    format!("{}:{}", peer.address, peer.port.to_string())
}

fn get_peer(key: &str) -> Option<PeerConfig> {
    let db = PEERSDB.lock().unwrap();
    let ret = db.get(&key.to_string()).cloned();
    drop(db);
    ret
}

fn set_peer(key: &str, peer: PeerConfig) {
    let mut db = PEERSDB.lock().unwrap();
    db.insert(key.to_string(), peer);
    drop(db);
}
/// Add a file to the database and asign a bufermap with 1 used in upload
pub fn add_seed_file_to_db(file: MetaFile) {
    todo!();
}
/// Add a file to the database and asign a bufermap with 0 used in download
pub fn add_leeched_file_to_db(file: MetaFile) {
    todo!();
}
/// Associate a peer with a key in the database
pub fn add_peer_to_key(config: PeerConfig, key: String) {
    todo!();
}
/// Update the buffermap of a peer on a file
pub fn update_buffermap(config: PeerConfig, key: String, buffermap: Vec<u8>) {
    todo!();
}
/// get buffermap to share it among other peers
pub fn get_buffermap(config: PeerConfig, key: String) {
    todo!();
}

pub fn get_peer_from_file(key: String) {
    todo!();
}

/// Add a file to the database and asign a bufermap with 1 used in upload
pub fn remove_file_from_db(file: MetaFile) {
    todo!();
}

/// Associate a peer with a key in the database
pub fn remove_peer_to_key(config: PeerConfig, key: String) {
    todo!();
}
pub fn log_db() {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_peer_key() {
        let peer = PeerConfig {
            address: "1.1.1.1".to_string(),
            port: 1234,
        };
        let expected = "1.1.1.1:1234".to_string();
        let result = get_peer_key(peer);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_get_peer() {
        // println!("test");
        let peer = PeerConfig {
            address: "1.1.1.1".to_string(),
            port: 1234,
        };
        let emptypeer = PeerConfig {
            address: "".to_string(),
            port: 0,
        };
        let mut db = PEERSDB.lock().unwrap();
        db.insert("1.1.1.1:1234".to_string(), peer.clone());
        // db.clear();
        drop(db);
        let result = match get_peer("1.1.1.1:1234") {
            Some(value) => value.clone(),
            None => emptypeer,
        };
        assert_eq!(result.address, peer.address);
    }
    #[test]
    fn test_set_peer() {
        let peer = PeerConfig {
            address: "1.1.1.1".to_string(),
            port: 1234,
        };
        let emptypeer = PeerConfig {
            address: "".to_string(),
            port: 0,
        };
        let key = "1.1.1.1:1234";
        set_peer(key, peer);
        let mut db = PEERSDB.lock().unwrap();
        let result = match db.get("1.1.1.1:1234") {
            Some(value) => value.clone(),
            None => emptypeer,
        };
        db.clear();
        drop(db);
        assert_eq!(result.port, 1234);
    }
}
