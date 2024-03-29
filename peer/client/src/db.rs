use crate::data::{MetaFile};
use log{info};
/// Add a file to the database and asign a bufermap with 1 used in upload
pub fn add_seed_file_to_db(file: MetaFile) {
    todo!();
}
/// Add a file to the database and asign a bufermap with 0 used in download
pub fn add_leeched_file_to_db(file: MetaFile) {
    todo!();
}
/// Associate a peer with a key in the database
pub fn add_peer_to_key(config: PeerConfig, key:String) {
    todo!();
}
/// Update the buffermap of a peer on a file
pub fn update_buffermap(config: PeerConfig, key: String, buffermap: vec<u8>) {
    todo!();
}
/// get buffermap to share it among other peers
pub fn get_buffermap(config: PeerConfig, key: String) {
    todo!();
}

pub fn get_peer_from_file(key:String) {
    todo!();
}

/// Add a file to the database and asign a bufermap with 1 used in upload
pub fn remove_file_from_db(file: MetaFile) {
    todo!();
}

/// Associate a peer with a key in the database
pub fn remove_peer_to_key(config: PeerConfig, key:String) {
    todo!();
}
pub log_db() {todo!();}
