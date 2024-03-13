// src/main.rs
mod userinput;
mod com;
mod config;

use userinput::{get_listen_port, get_proposed_files};
use com::send_port_seed_to_tracker;
use config::Config;



fn main() {
    let mut config = Config {
        port: 8080,
        files: Vec::new(),
    };
    // nul si le port change pas et en plus ça marche pas
    config.port = get_listen_port();
    config.files = get_proposed_files();
    send_port_seed_to_tracker(config.port, config.files);
    // Idee faire des mocks du server en simulant les echanges TCP avec des réponses constantes
}
