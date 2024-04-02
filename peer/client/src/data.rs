use ini::Ini;
use md5::{Digest, Md5};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
#[derive(Debug)]
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

        let peer_address = peer_section.get("peer-address").unwrap().to_string();
        let peer_port = peer_section
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
pub fn get_file_key(path: &str) -> String {
    let path = Path::new(path);
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut hasher = Md5::new();
    std::io::copy(&mut reader, &mut hasher).unwrap();
    let result = hasher.finalize();
    format!("{:x}", result)
}
