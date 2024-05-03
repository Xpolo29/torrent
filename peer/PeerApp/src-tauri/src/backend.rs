mod back;
mod com;
mod data;
mod db;
mod menu;
mod parser;
mod process;
mod respons_handler;
mod tasks;
mod userinput;
mod threads;
use data::{TrackerConfig, PeerConfig};
use menu::display_menu;
use simplelog::*;
use std::fs::File;
//use tasks::EmptyTask;
use threads::Pool;
use std::mem;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![openConnection, closeConnection, uploadFiles])
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application!");
}

// add before every function
#[tauri::command]
fn openConnection() {
    //create pool
    let mut pool: Pool = Pool::new(2);

    //start update thread
    let mut tracker_config = TrackerConfig::new();
    pool.start_update(tracker_config, 30);

    //start listening thread
    let peer_config = PeerConfig::from_config();
    pool.start_listening(peer_config);

    let log_file = File::create("client.log").unwrap();

    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Trace,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(LevelFilter::Trace, Config::default(), log_file),
    ])
    .unwrap();

    let pool_clone = pool.clone();

    let tracker_config = TrackerConfig::new();
    display_menu(tracker_config, pool_clone);
}

#[tauri::command]
fn closeConnection(pool: Pool) {
    // Close the pool
    mem::drop(pool);
}

#[tauri::command]
fn uploadFiles(seeded_files: String) {
    let peer_config = PeerConfig::from_config();

    let tracker_port = peer_config.port;
    let tracker_address = peer_config.address;

    let seeded_files: Vec<MetaFile> = seeded_files
        .into_iter()
        .map(|file| MetaFile::new(file.to_string()))
        .collect(); // Create vector of Metafiles out of the files name

    let seeded_files2 = seeded_files.clone();
        for seed in seeded_files2 {
            add_seed_file_to_db(seed);
        }

    let seeded_files = seed(seeded_files, peer_config.port.to_string(), "".to_string()); // create the message
    trace!("Prepared message: {}", seeded_files);
    if let Some(mut stream) = connect(tracker_port, &tracker_address.to_string()) {
        // connect to the tracker
        send(&mut stream, seeded_files); // send the message
        trace!("Message sent waiting for answer");
        let response = receive(&mut stream); // receive the answer
        trace!("Received: {}", response);
        match ExpectOk.check_answer(&response) {
            Ok(valeur) => {
                info!("{}", valeur);
            }
            Err(valeur) => {
                info!("{}", valeur);
            }
        }
        ExpectOk.shutdown(&mut stream);
    }
}

