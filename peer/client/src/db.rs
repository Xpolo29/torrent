use crate::data::*;
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
//FILEDB and PEERSDB might be useless
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
    // drop(db);
    ret
}

fn set_peer(key: &str, peer: PeerConfig) {
    let mut db = PEERSDB.lock().unwrap();
    db.insert(key.to_string(), peer);
    // drop(db);
}

fn get_file(key: &str) -> Option<MetaFile> {
    let db = FILEDB.lock().unwrap();
    let ret = db.get(&key.to_string()).cloned();
    // drop(db);
    ret
}

fn set_file(key: &str, file: MetaFile) {
    let mut db = FILEDB.lock().unwrap();
    db.insert(key.to_string(), file);
    // drop(db);
}

fn set_buffermap(file_key: String, peer_key: String, buffermap: Vec<u8>) {
    let mut buffermap_db = BUFFERMAPDB.lock().unwrap();
    let file_buffermaps = buffermap_db
        .entry(file_key.clone())
        .or_insert_with(HashMap::new);
    file_buffermaps.insert(peer_key, buffermap);
    drop(buffermap_db);
}

fn __get_buffermap(file_key: &str, peer_key: &str) -> Option<Vec<u8>> {
    let buffermap_db = BUFFERMAPDB.lock().unwrap();
    let file_buffermaps = buffermap_db.get(file_key)?;
    let buffermap = file_buffermaps.get(peer_key)?;
    Some(buffermap.clone())
}

/// Add a file to the database and asign a bufermap with 1 used in upload
pub fn add_seed_file_to_db(file: MetaFile) {
    let file_key = get_file_hash(&file);
    let me = PeerConfig::from_config();
    let peer_key = get_peer_key(me);
    let buffersize = get_buffer_size(&file) as usize;
    let buffermap = vec![1u8; buffersize];
    set_file(file_key.as_str(), file);
    set_buffermap(file_key, peer_key, buffermap)
}
/// Add a file to the database and asign a bufermap with 0 used in download
pub fn add_leeched_file_to_db(file: MetaFile) {
    let file_key = get_file_hash(&file);
    let me = PeerConfig::from_config();
    let peer_key = get_peer_key(me);
    let buffersize = get_buffer_size(&file) as usize;
    let buffermap = vec![0u8; buffersize];
    set_file(file_key.as_str(), file);
    set_buffermap(file_key, peer_key, buffermap)
}
/// Associate a peer with a key in the database with a buffermap, can be used to update the buffermap
/// totest
pub fn set_peer_to_file(config: PeerConfig, key: String, buffermap: Vec<u8>) {
    let peer_key = get_peer_key(config);
    set_buffermap(key, peer_key, buffermap);
}

/// get buffermap to share it among other peers
pub fn get_buffermap(config: PeerConfig, key: &str) -> Option<Vec<u8>> {
    __get_buffermap(key, &get_peer_key(config))
}

///Totest
pub fn get_peer_from_file(key: String) -> Vec<PeerConfig> {
    let buffermap_db = BUFFERMAPDB.lock().unwrap();
    let file_buffermaps = buffermap_db.get(&key);
    let mut peers: Vec<PeerConfig> = vec![];
    if let Some(file_buffermaps) = file_buffermaps {
        for (peer_key, _) in file_buffermaps {
            let peer_parts: Vec<&str> = peer_key.split(':').collect();
            let address = peer_parts[0].to_string();
            let port = peer_parts[1].parse().unwrap();
            peers.push(PeerConfig { address, port });
        }
    }
    peers
}

/// Add a file to the database and asign a bufermap with 1 used in upload
pub fn remove_file_from_db(file: MetaFile) {
    let mut file_db = FILEDB.lock().unwrap();
    let mut buffermap_db = BUFFERMAPDB.lock().unwrap();
    if let Some(file_key) = file_db
        .clone()
        .keys()
        .find(|key| key.to_string() == file.hash)
    {
        file_db.remove(file_key);
        buffermap_db.remove(file_key);
    }
}

/// Associate a peer with a key in the database
/// totest
pub fn remove_peer_to_file(config: PeerConfig, key: String) {
    let mut buffermap_db = BUFFERMAPDB.lock().unwrap();
    let peer_key = get_peer_key(config);
    if let Some(file_buffermaps) = buffermap_db.get_mut(&key) {
        file_buffermaps.remove(&peer_key);
    }
}
pub fn log_db() {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_file_from_db() {
        let meta = MetaFile {
            file_name: "test".to_string(),
            length: 10,
            piece_size: 10,
            hash: "hash".to_string(),
        };
        let file1 = "hash";
        let file2 = "hash2";
        let peer = "1.1.1.1:1234";
        let buffermap = vec![1u8, 10];
        set_buffermap(file1.to_string(), peer.to_string(), buffermap.clone());
        set_buffermap(file2.to_string(), peer.to_string(), buffermap.clone());
        let result = __get_buffermap(file1, peer);
        assert_eq!(result.unwrap(), buffermap);
        let result = __get_buffermap(file2, peer);
        assert_eq!(result.unwrap(), buffermap);
        remove_file_from_db(meta);
        let result = __get_buffermap(peer, file1);
        assert!(result.is_none());
        let result = __get_buffermap(file2, peer);
        assert_eq!(result.unwrap(), buffermap);
    }

    #[test]
    fn test_get_buffermap() {
        let peer = "1.1.1.1:1234";
        let file = "hash";
        let buffermap: Vec<u8> = vec![1u8; 10];
        set_buffermap(file.to_string(), peer.to_string(), buffermap.clone());
        let result = __get_buffermap(file, peer).unwrap();
        assert_eq!(result, buffermap);
    }

    #[test]
    fn test_set_buffermap() {
        let peer = "1.1.1.1:1234";
        let peer2 = "2.2.2.2:1234";
        let file = "hash";
        let file2 = "hash2";
        let buffermap: Vec<u8> = vec![1u8; 10];
        let buffermap2: Vec<u8> = vec![0u8; 10];
        set_buffermap(file.to_string(), peer.to_string(), buffermap.clone());
        let db = BUFFERMAPDB.lock().unwrap();
        let mut db_file = db.get(file).unwrap();
        let mut result = db_file.get(peer);
        assert_eq!(result.unwrap(), &buffermap);
        drop(db);
        // drop(db);

        set_buffermap(file2.to_string(), peer2.to_string(), buffermap2.clone());
        let db = BUFFERMAPDB.lock().unwrap();
        db_file = db.get(file2).unwrap();
        result = db_file.get(peer2);
        assert_eq!(result.unwrap(), &buffermap2);
        drop(db);
    }

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
