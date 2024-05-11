use crate::data::*;
use hashbrown::HashMap;
use lazy_static::lazy_static;
//use log::{debug, error, info};
use rayon::prelude::*;
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

/// Generates a unique key for a peer.
///
/// This function takes a PeerConfig struct.
/// It formats the peer's address and port into a string, separated by a colon.
/// It then returns this string.
///
/// # Arguments
/// * `peer` - A PeerConfig struct.
///
/// # Returns
/// * `String` - The unique key for the peer.
pub fn get_peer_key(peer: PeerConfig) -> String {
    format!("{}:{}", peer.address, peer.port.to_string())
}

/// Retrieves a peer from the database.
///
/// This function takes a key as a string slice.
/// It locks the database, retrieves the peer associated with the key, and clones it.
/// It then returns the cloned peer, or None if no peer was found for the key.
///
/// # Arguments
/// * `key` - A string slice representing the key.
///
/// # Returns
/// * `Option<PeerConfig>` - The peer associated with the key, or None if no peer was found.
pub fn get_peer(key: &str) -> Option<PeerConfig> {
    let db = PEERSDB.lock().unwrap();
    let ret = db.get(&key.to_string()).cloned();
    // drop(db);
    ret
}

/// Inserts a peer into the database.
///
/// This function takes a key as a string slice and a PeerConfig struct.
/// It locks the database and inserts the peer into the database with the key.
///
/// # Arguments
/// * `key` - A string slice representing the key.
/// * `peer` - A PeerConfig struct.
fn set_peer(key: &str, peer: PeerConfig) {
    let mut db = PEERSDB.lock().unwrap();
    db.insert(key.to_string(), peer);
    // drop(db);
}

/// Retrieves a file from the database.
///
/// This function takes a key as a string slice.
/// It locks the database, retrieves the file associated with the key, and clones it.
/// It then returns the cloned file, or None if no file was found for the key.
///
/// # Arguments
/// * `key` - A string slice representing the key.
///
/// # Returns
/// * `Option<MetaFile>` - The file associated with the key, or None if no file was found.
pub fn get_file(key: &str) -> Option<MetaFile> {
    let db = FILEDB.lock().unwrap();
    let ret = db.get(&key.to_string()).cloned();
    // drop(db);
    ret
}

/// Inserts a file into the database.
///
/// This function takes a key as a string slice and a MetaFile struct.
/// It locks the database and inserts the file into the database with the key.
///
/// # Arguments
/// * `key` - A string slice representing the key.
/// * `file` - A MetaFile struct.
fn set_file(file: MetaFile) {
    let key: &str = &file.hash;
    let mut db = FILEDB.lock().unwrap();
    db.insert(key.to_string(), file);
    // drop(db);
}

/// Inserts a buffermap into the database.
///
/// This function takes a file key, a peer key, and a buffermap.
/// It locks the database and inserts the buffermap into the database with the file key and peer key.
///
/// # Arguments
/// * `file_key` - A String representing the file key.
/// * `peer_key` - A String representing the peer key.
/// * `buffermap` - A Vec<u8> representing the buffermap.
pub fn set_buffermap(file_key: String, peer_key: String, buffermap: Vec<u8>) {
    let mut buffermap_db = BUFFERMAPDB.lock().unwrap();
    let file_buffermaps = buffermap_db
        .entry(file_key.clone())
        .or_insert_with(HashMap::new);
    if let Some(buf) = file_buffermaps.get_mut(&peer_key) {
        modify_buffer(buf, buffermap);
    } else {
        file_buffermaps.insert(peer_key, buffermap);
    }
    drop(buffermap_db);
}

fn modify_buffer(bufdest: &mut Vec<u8>, bufsrc: Vec<u8>) {
    bufdest.copy_from_slice(&bufsrc);
    /*
    for (i, &src_byte) in bufsrc.iter().enumerate() {
        bufdest[i] = src_byte;
    }
    */
}

/// Retrieves a buffermap from the database.
///
/// This function takes a file key and a peer key.
/// It locks the database, retrieves the buffermap associated with the file key and peer key, and clones it.
/// It then returns the cloned buffermap, or None if no buffermap was found for the file key and peer key.
///
/// # Arguments
/// * `file_key` - A string slice representing the file key.
/// * `peer_key` - A string slice representing the peer key.
///
/// # Returns
/// * `Option<Vec<u8>>` - The buffermap associated with the file key and peer key, or None if no buffermap was found.
fn __get_buffermap(file_key: &str, peer_key: &str) -> Option<Vec<u8>> {
    let buffermap_db = BUFFERMAPDB.lock().unwrap();
    let file_buffermaps = buffermap_db.get(file_key)?;
    let buffermap = file_buffermaps.get(peer_key)?;
    Some(buffermap.clone())
}

/// Adds a seed file to the database.
///
/// This function takes a MetaFile struct.
/// It computes the file's hash and uses it as a key to store the file in the database.
/// It also creates a buffermap filled with 1s and stores it in the database with the file key and the peer key.
///
/// # Arguments
/// * `file` - A MetaFile struct representing the file to be added.
pub fn add_seed_file_to_db(file: MetaFile) {
    // add file to db
    let file_key = get_file_hash(&file);
    set_file(file.clone());

    // add peer to db
    let me = PeerConfig::new();
    let me2 = me.clone();
    let peer_key = get_peer_key(me);
    let buffersize = get_buffer_size(&file) as usize;
    set_peer(&peer_key, me2);

    // add buffermap to db
    let buffermap = vec![1u8; buffersize];
    set_buffermap(file_key, peer_key, buffermap)
}

/// Adds a leeched file to the database.
///
/// This function takes a MetaFile struct.
/// It computes the file's hash and uses it as a key to store the file in the database.
/// It also creates a buffermap filled with 0s and stores it in the database with the file key and the peer key.
///
/// # Arguments
/// * `file` - A MetaFile struct representing the file to be added.
pub fn add_leeched_file_to_db(file: MetaFile) {
    let file_key = get_file_hash(&file);
    set_file(file.clone());
    let me = PeerConfig::new();
    let peer_key = get_peer_key(me);
    let buffersize = get_buffer_size(&file) as usize;
    let buffermap = vec![0u8; buffersize];
    //set_file(file);
    set_buffermap(file_key, peer_key, buffermap)
}

/// Associates a peer with a file in the database and sets a buffermap for the file.
///
/// This function can also be used to update the buffermap for a file.
///
/// # Arguments
/// * `config` - A PeerConfig struct representing the peer.
/// * `file` - A MetaFile struct representing the file.
/// * `buffermap` - A Vec<u8> representing the buffermap.
pub fn set_peer_to_file(config: PeerConfig, file: MetaFile, buffermap: Vec<u8>) {
    let peer_key = get_peer_key(config.clone());
    set_peer(&file.hash, config);
    set_file(file.clone());
    set_buffermap(file.hash, peer_key, buffermap);
}

/// Retrieves a buffermap for a file from the database.
///
/// This function can be used to share the buffermap among other peers.
///
/// # Arguments
/// * `config` - A PeerConfig struct representing the peer.
/// * `key` - A string slice representing the key for the file.
///
/// # Returns
/// * `Option<Vec<u8>>` - The buffermap for the file, or None if no buffermap was found.
pub fn get_buffermap(config: PeerConfig, key: &str) -> Option<Vec<u8>> {
    __get_buffermap(key, &get_peer_key(config))
}

/// Retrieves all peers associated with a file from the database.
///
/// # Arguments
/// * `key` - A String representing the key for the file.
///
/// # Returns
/// * `Vec<PeerConfig>` - A vector of PeerConfig structs representing the peers.
pub fn get_peers_from_file(key: String) -> Vec<PeerConfig> {
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

/// Removes a file from the database and its associated buffermap.
///
/// This function takes a MetaFile struct.
/// It locks the file database and the buffermap database, finds the file in the file database using its hash, and removes it.
/// It also removes the associated buffermap from the buffermap database.
///
/// # Arguments
/// * `file` - A MetaFile struct representing the file to be removed.
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

/// Disassociates a peer from a file in the database.
///
/// This function takes a PeerConfig struct and a key.
/// It locks the buffermap database, finds the buffermap for the file using the key,
/// and removes the peer from the buffermap.
///
/// # Arguments
/// * `config` - A PeerConfig struct representing the peer.
/// * `key` - A String representing the key for the file.
pub fn remove_peer_to_file(config: PeerConfig, key: String) {
    let mut buffermap_db = BUFFERMAPDB.lock().unwrap();
    let peer_key = get_peer_key(config);
    if let Some(file_buffermaps) = buffermap_db.get_mut(&key) {
        file_buffermaps.remove(&peer_key);
    }
}

/// Removes a peer from the database.
///
/// This function takes a PeerConfig struct.
/// It locks the peer database and the buffermap database, and removes the peer from both databases.
///
/// # Arguments
/// * `config` - A PeerConfig struct representing the peer.
pub fn remove_peer_from_db(config: PeerConfig) {
    let key = get_peer_key(config);

    // Remove the peer from the PEERSDB hash map
    let mut db = PEERSDB.lock().unwrap();
    db.remove(&key);
    drop(db);

    // Remove the peer from all file buffer maps in the BUFFERMAPDB hash map
    let mut db = BUFFERMAPDB.lock().unwrap();
    for (_file_key, file_buffermap) in db.iter_mut() {
        file_buffermap.remove(&key);
    }
    drop(db);
}

/// Clears all entries from the buffermap, file, and peer databases.
///
/// This function locks each database and replaces its contents with a new, empty HashMap.
fn clear_db() {
    let mut buffermap_db = BUFFERMAPDB.lock().unwrap();
    let mut file_db = FILEDB.lock().unwrap();
    let mut peer_db = PEERSDB.lock().unwrap();
    *buffermap_db = HashMap::new();
    *file_db = HashMap::new();
    *peer_db = HashMap::new();
}

/// Retrieves all files that a specific peer has a non-empty buffermap for.
///
/// This function takes a PeerConfig struct, locks the buffermap database, and iterates over all file buffermaps.
/// If the peer has a non-empty buffermap for a file,
/// the function retrieves the MetaFile struct for the file from the file database and adds it to the result vector.
///
/// # Arguments
/// * `config` - A PeerConfig struct representing the peer.
///
/// # Returns
/// * `Vec<MetaFile>` - A vector of MetaFile structs representing the files.
pub fn get_file_from_peer(config: PeerConfig) -> Vec<MetaFile> {
    let key = get_peer_key(config);
    let mut result = vec![];

    // Iterate over all file buffer maps in the BUFFERMAPDB hash map
    let db = BUFFERMAPDB.lock().unwrap();
    for (file_key, file_buffermap) in db.iter() {
        // Check if the peer has a non-empty buffer map for this file
        if let Some(buffermap) = file_buffermap.get(&key) {
            if !buffermap.is_empty() {
                // Get the MetaFile struct for this file from the FILEDB hash map
                let file_db = FILEDB.lock().unwrap();
                if let Some(meta_file) = file_db.get(file_key) {
                    result.push(meta_file.clone());
                }
            }
        }
    }

    result
}

/// Logs the contents of the buffermap database.
///
/// This function locks the buffermap database and prints its contents to the console.
pub fn log_db() {
    let buffermap_db = BUFFERMAPDB.lock().unwrap();
    println!("BUFFERMAPDB:");
    for (file_key, file_buffermaps) in buffermap_db.iter() {
        println!("  {}:", file_key);
        for (peer_key, buffermap) in file_buffermaps.iter() {
            println!("    {}: {:?}", peer_key, buffermap);
        }
    }
}

/// Retrieves all files that the local peer is seeding.
///
/// This function locks the buffermap database and iterates over all file buffermaps.
/// If the local peer has a buffermap filled with 1 for a file, the function retrieves the MetaFile struct for the file from the file database and adds it to the result vector.
///
/// # Returns
/// * `Vec<MetaFile>` - A vector of MetaFile structs representing the files.
pub fn get_seeding_files() -> Vec<MetaFile> {
    let me = PeerConfig::new();
    let mut result = vec![];

    // Iterate over all file buffer maps in the BUFFERMAPDB hash map
    let db = BUFFERMAPDB.lock().unwrap();
    for (file_key, file_buffermap) in db.iter() {
        // Check if the local peer has a buffer map filled with 1 for this file
        if let Some(buffermap) = file_buffermap.get(&get_peer_key(me.clone())) {
            if buffermap.iter().all(|b| *b == 1) {
                // Get the MetaFile struct for this file from the FILEDB hash map
                let file_db = FILEDB.lock().unwrap();
                if let Some(meta_file) = file_db.get(file_key) {
                    result.push(meta_file.clone());
                }
            }
        }
    }

    result
}

/// Retrieves all files that the local peer is leeching.
///
/// This function locks the buffermap database and iterates over all file buffermaps.
/// If the local peer has a buffermap with at least one 0 for a file, the function retrieves the MetaFile struct for the file from the file database and adds it to the result vector.
///
/// # Returns
/// * `Vec<MetaFile>` - A vector of MetaFile structs representing the files.
pub fn get_leeching_files() -> Vec<MetaFile> {
    let me = PeerConfig::new();
    let mut result = vec![];

    // Iterate over all file buffer maps in the BUFFERMAPDB hash map
    let db = BUFFERMAPDB.lock().unwrap();
    for (file_key, file_buffermap) in db.iter() {
        // Check if the local peer has a buffer map with at least one "0" for this file
        if let Some(buffermap) = file_buffermap.get(&get_peer_key(me.clone())) {
            if buffermap.iter().any(|b| *b == 0) {
                // Get the MetaFile struct for this file from the FILEDB hash map
                let file_db = FILEDB.lock().unwrap();
                if let Some(meta_file) = file_db.get(file_key) {
                    result.push(meta_file.clone());
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_seeding_or_leeching() {
        let meta = MetaFile {
            file_name: "test".to_string(),
            length: 10,
            piece_size: 10,
            hash: "hash".to_string(),
        };
        let meta2 = MetaFile {
            file_name: "test2".to_string(),
            length: 10,
            piece_size: 10,
            hash: "hash2".to_string(),
        };
        let meta3 = MetaFile {
            file_name: "test3".to_string(),
            length: 10,
            piece_size: 10,
            hash: "hash3".to_string(),
        };
        let me = PeerConfig::new();
        let mut buffermap: Vec<u8> = vec![1u8; 10];
        buffermap[0] = 0;
        add_seed_file_to_db(meta);
        add_leeched_file_to_db(meta2);
        // add_leeched_file_to_db(meta3.clone());
        set_peer_to_file(me, meta3, buffermap);

        let result = get_seeding_files();
        assert_eq!(result[0].hash, "hash");
        let result = get_leeching_files();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].hash, "hash2");
        assert_eq!(result[1].hash, "hash3");
        clear_db();
    }

    #[test]
    fn test_remove_peer_to_file() {
        clear_db();
        let peer1 = PeerConfig {
            address: "1.1.1.1".to_string(),
            port: 1234,
        };
        let peer2 = PeerConfig {
            address: "2.2.2.2".to_string(),
            port: 1234,
        };
        let meta = MetaFile {
            file_name: "test".to_string(),
            length: 10,
            piece_size: 10,
            hash: "hash1".to_string(),
        };

        let buffermap = vec![1u8; 10];
        let buffermap2 = vec![0u8; 10];
        set_peer_to_file(peer1.clone(), meta.clone(), buffermap);
        set_peer_to_file(peer2.clone(), meta.clone(), buffermap2.clone());
        assert!(!get_peers_from_file("hash1".to_string()).is_empty());
        assert_eq!(get_peers_from_file("hash1".to_string()).len(), 2);
        remove_peer_to_file(peer1, "hash1".to_string());
        assert!(!get_peers_from_file("hash1".to_string()).is_empty());
        assert_eq!(get_peers_from_file("hash1".to_string()).len(), 1);
        clear_db();
    }

    #[test]
    fn test_get_peer_from_file() {
        clear_db();
        let peer1 = PeerConfig {
            address: "1.1.1.1".to_string(),
            port: 1234,
        };
        let peer2 = PeerConfig {
            address: "2.2.2.2".to_string(),
            port: 1234,
        };
        let peer3 = PeerConfig {
            address: "3.3.3.3".to_string(),
            port: 1234,
        };
        let meta = MetaFile {
            file_name: "test".to_string(),
            length: 10,
            piece_size: 10,
            hash: "hash1".to_string(),
        };
        let buffermap = vec![1u8; 10];
        let buffermap2 = vec![0u8; 10];
        set_peer_to_file(peer1.clone(), meta.clone(), buffermap);
        set_peer_to_file(peer2.clone(), meta.clone(), buffermap2.clone());
        let result = get_peers_from_file("hash1".to_string());
        assert!(!result.is_empty());
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].address, "1.1.1.1".to_string());
        assert_eq!(result[1].address, "2.2.2.2".to_string());
        set_peer_to_file(peer3.clone(), meta, buffermap2);
        let result = get_peers_from_file("hash1".to_string());
        assert!(!result.is_empty());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].address, "1.1.1.1".to_string());
        assert_eq!(result[1].address, "3.3.3.3".to_string());
        assert_eq!(result[2].address, "2.2.2.2".to_string());
        clear_db();
    }
    #[test]
    fn test_set_peer_to_file() {
        clear_db();
        let peer1 = PeerConfig {
            address: "1.1.1.1".to_string(),
            port: 1234,
        };
        let meta = MetaFile {
            file_name: "test".to_string(),
            length: 10,
            piece_size: 10,
            hash: "hash1".to_string(),
        };
        let buffermap = vec![1u8; 10];
        set_peer_to_file(peer1.clone(), meta, buffermap);
        assert!(get_buffermap(peer1, "hash1").is_some());
        clear_db();
    }

    #[test]
    fn test_remove_file_from_db() {
        clear_db();
        let meta = MetaFile {
            file_name: "test".to_string(),
            length: 10,
            piece_size: 10,
            hash: "hash".to_string(),
        };
        let file1 = "hash";
        let file2 = "hash2";
        let peer = "1.1.1.1:1234";
        let buffermap = vec![1u8; 10];
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
        clear_db();
    }

    #[test]
    fn test_get_buffermap() {
        clear_db();
        let peer = "1.1.1.1:1234";
        let file = "hash";
        let buffermap: Vec<u8> = vec![1u8; 10];
        set_buffermap(file.to_string(), peer.to_string(), buffermap.clone());
        let result = __get_buffermap(file, peer).unwrap();
        assert_eq!(result, buffermap);
        clear_db();
    }

    #[test]
    fn test_set_buffermap() {
        clear_db();
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
        log_db();

        set_buffermap(file2.to_string(), peer2.to_string(), buffermap2.clone());
        let db = BUFFERMAPDB.lock().unwrap();
        db_file = db.get(file2).unwrap();
        result = db_file.get(peer2);
        assert_eq!(result.unwrap(), &buffermap2);
        drop(db);
        clear_db();
    }

    #[test]
    fn test_get_peer_key() {
        clear_db();
        let peer = PeerConfig {
            address: "1.1.1.1".to_string(),
            port: 1234,
        };
        let expected = "1.1.1.1:1234".to_string();
        let result = get_peer_key(peer);
        assert_eq!(result, expected);
        clear_db();
    }
    #[test]
    fn test_get_peer() {
        clear_db();
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
        clear_db();
    }
    #[test]
    fn test_set_peer() {
        clear_db();
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
        clear_db();
    }
}
