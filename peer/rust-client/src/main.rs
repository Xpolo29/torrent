// src/main.rs
mod com;
mod network_config;
mod parse;
mod userinput;
use com::{send_port_seed_to_tracker, send_search_to_tracker};
use network_config::PeerConfig;
use userinput::{get_available_files, get_file, get_listen_port, get_proposed_files};

fn main() {
    let interactive = false;
    let mut peer_config = PeerConfig::new();
    // nul si le port change pas et en plus ça marche pas
    if interactive {
        peer_config.port = get_listen_port();
        peer_config.files = get_proposed_files();
    } else {
        // let args: Vec<String> = std::env::args().collect();
    }
    send_port_seed_to_tracker(peer_config.port, peer_config.files);
    get_available_files();
    send_search_to_tracker(get_available_files());
    get_file();
    /*
     */
}
