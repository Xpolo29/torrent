use std::collections::HashMap;

pub struct File {
    filename: String,
    length: i32,
    piece_size: i32,
    key: String,
    buffersize: i32,
}

pub struct Peer {
    ip: String,
    port: i32,
    key: String,
    buffermap: Option<Vec<u8>>,
}

pub struct Database {
    files: HashMap<String, File>,
    peers: HashMap<(String, i32, String), Peer>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
            peers: HashMap::new(),
        }
    }

    // FILE //

    pub fn get_file(&self, key: &str) -> Option<&File> {
        self.files.get(key)
    }

    pub fn insert_file(&mut self, filename: String, length: i32, piece_size: i32, key: String) {
        let buffersize = ((length as f32 / piece_size as f32).ceil() as i32) / 8;
        let file = File {
            filename,
            length,
            piece_size,
            key: key.clone(),
            buffersize,
        };
        self.files.insert(key, file);
    }

    pub fn delete_file(&mut self, key: &str) {
        self.files.remove(key);
    }

    // PEER //

    pub fn get_peer(&self, ip: &str, port: i32, key: &str) -> Option<&Peer> {
        self.peers.get(&(ip.to_string(), port, key.to_string()))
    }
    
    pub fn insert_peer_no_buffermap(&mut self, ip: String, port: i32, key: String) {
        let peer = Peer {
            ip: ip.clone(),
            port,
            key: key.clone(),
            buffermap: None,
        };
        self.peers.insert((ip, port, key), peer);
    }
    
    // size of buffermap can chnage, and is found in buffersize in file struct
    pub fn insert_peer_with_buffermap(&mut self, ip: String, port: i32, key: String, buffermap: Vec<u8> ) {
        let buffersize = self.files.get(&key).map_or(0, |file| file.buffersize);
        // FIXME
        let peer = Peer {
            ip: ip.clone(),
            port,
            key: key.clone(),
            buffermap: buffermap,
        };
        self.peers.insert((ip, port, key), peer);
    }
    
    pub fn delete_peer(&mut self, ip: &str, port: i32, key: &str) {
        self.peers.remove(&(ip.to_string(), port, key.to_string()));
    }

    pub fn get_peer_buffermap(&self, ip: &str, port: i32, key: &str) -> Option<&Vec<u8>> {
        self.peers.get(&(ip.to_string(), port, key.to_string()))?.buffermap.as_ref()
    }

    pub fn update_peer_buffermap(&mut self, ip: String, port: i32, key: String, new_buffermap: Vec<u8>) {
        if let Some(peer) = self.peers.get_mut(&(ip.clone(), port, key.clone())) {
            peer.buffermap = Some(new_buffermap);
            return 0;
        }
        return -1;
    }
    
}