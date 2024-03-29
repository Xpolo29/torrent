use crate::data::{MetaFile, PeerConfig};
use crate::com::{connect, receive, send, get_file_request};
use crate::respons_handler::{ExpectPeers}
use crate::db::*;
fn start_download(key:String) {
    let peers = get_peer_from_file(key);
    if let Some(mut stream) = connect(tracker_port, &tracker_adress.to_string()) {
    let message = getfile_request(key);
    send(&mut stream, message);
    let response = receive(&mut stream);
    
}
}