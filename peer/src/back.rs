use crate::com::{connect, getfile_request, receive, send};
use crate::data::PeerConfig;
use crate::db::*;

fn start_download(key: String) {
    let peers = get_peer_from_file(key); // we dont use peer and we connect to the tracker ?
    if let Some(mut stream) = connect(tracker_port, &tracker_adress.to_string()) {
        let message = getfile_request(key);
        send(&mut stream, message);
        let response = receive(&mut stream);
    }
}
/// takes a file and help know which peer to ask for what piece
fn get_peer_and_piece_indices(key: String) -> Vec<(PeerConfig, Vec<u32>)> {
    // fetch buffermap from all leechers/seeders that have it
    // push the rarest piece to the beginning of the vector
    todo!();
}
