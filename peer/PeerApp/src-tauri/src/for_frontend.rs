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


// add #[tauri::command] before every function to be used in javascript
// functions to be defined
// get data for dashboard (files, download percentage, peers, leeching status )
// backend for upload files
// backend for download (search) files
// search section menu.rs - 

#[tauri::command]
pub fn searchFunction(filename: String, filesize: String) -> String {
    let look_message = lookf(&filename, &filesize);   
    trace!("Prepared message: {}", look_message);
    let mut present_files: Answer = Answer::List(Vec::new());
    let mut ret: Answer = Answer::List(Vec::new());
    if let Some(mut stream) = connect(tracker_port, &tracker_adress.to_string()) {
        send(&mut stream, look_message);
        trace!("Message sent waiting for answer");
        let response = receive(&mut stream);
        trace!("Received {}", response);

        match ExpectList.check_answer(&response) {
            Ok(valeur) => {
                info!("{}", valeur);
                present_files = ExpectList.retrieve_data(response.clone());
                ret = ExpectList.retrieve_data(response);
            }
            Err(valeur) => {
                error!("{}", valeur);
            }
        }
    }
    info!("files retrieved {:?}", present_files);

    // return string of files
    let file_strings: Vec<String> = match present_files {
        Answer::List(metafiles) => {
            metafiles.iter().map(|file| {
                let buffmap: Vec<u8> = vec![0; get_buffer_size(&file)];
                let conf: PeerConfig = PeerConfig::new();
                set_peer_to_file(conf, file.clone(), buffmap);
                format!("{}#{}#{}", file.filename, file.length, file.hash)
            }).collect()
        }
        _ => {
            error!("Could not add filelist to db");
            Vec::new()
        }
    };

    let files = file_strings.join(" | ");
    files
}
