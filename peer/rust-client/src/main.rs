// src/main.rs
mod com;
mod config;
mod userinput;

use com::{send_port_seed_to_tracker, send_search_to_tracker};
use config::Config;
use userinput::{get_available_files, get_listen_port, get_proposed_files};

fn main() {
    let mut config = Config {
        port: 8080,
        files: Vec::new(),
    };
    // nul si le port change pas et en plus ça marche pas

    config.port = get_listen_port();
    config.files = get_proposed_files();
    send_port_seed_to_tracker(config.port, config.files);
    let desired_files = get_available_files();
    send_search_to_tracker(desired_files);
}
