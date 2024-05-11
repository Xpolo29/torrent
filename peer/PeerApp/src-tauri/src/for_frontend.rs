use crate::back::start_download;
use crate::com::{connect, lookf, receive, seedf, send};
use crate::data::{get_buffer_size, get_file_key, MetaFile, PeerConfig, TrackerConfig};
use crate::db::*;
use crate::respons_handler::{Answer, ExpectList, ExpectOk, ExpectedAnswer};
use crate::tasks::EmptyTask;
use crate::threads::Pool;
use crate::userinput::{choose_file, get_file_names, get_filename, get_filesize};
use log::{error, info, trace, warn};
use std::fmt::Write;
use std::io;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;
enum Status {
    LEECHING = 0,
    SEEDING = 1,
}

// add #[tauri::command] before every function to be used in javascript
// functions to be defined
// get data for dashboard (files, download percentage, peers, leeching status )
// backend for upload files
// backend for download (search) files
// search section menu.rs -

#[tauri::command]
pub fn searchFunction(filename: String, filesize: String) -> String {
    let tracker = TrackerConfig::new();
    let tracker_port = tracker.port;
    let tracker_adress = tracker.address;
    let look_message = lookf(filename, filesize);
    trace!("Prepared message: {}", look_message);
    let mut present_files: Answer = Answer::List(Vec::new());
    let mut ret: Answer = Answer::List(Vec::new());
    if let Some(mut stream) = connect(tracker_port, &tracker_adress.to_string()) {
        send(&mut stream, look_message);
        trace!("Message sent waiting for answer");
        let response = receive(&mut stream, 3000);
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
        Answer::List(metafiles) => metafiles
            .iter()
            .map(|file| {
                let buffmap: Vec<u8> = vec![0; get_buffer_size(&file)];
                let conf: PeerConfig = PeerConfig::new();
                set_peer_to_file(conf, file.clone(), buffmap);
                format!("{}#{}#{}", file.file_name, file.length, file.hash)
            })
            .collect(),
        _ => {
            error!("Could not add filelist to db");
            Vec::new()
        }
    };

    let files = file_strings.join(" | ");
    files
}

fn count(vec: &Vec<u8>) -> (usize, usize) {
    let mut count_zero = 0;
    let mut count_one = 0;

    for &item in vec {
        match item {
            0 => count_zero += 1,
            1 => count_one += 1,
            _ => (),
        }
    }

    (count_zero, count_one)
}

pub fn get_all_files() -> Vec<MetaFile> {
    let mut files = get_seeding_files();
    let mut leeching = get_leeching_files().clone();
    files.append(&mut leeching);
    files
}

pub fn get_all_files_name() -> Vec<String> {
    let files = get_all_files();
    let mut names = Vec::new();
    for file in files {
        names.push(file.file_name);
    }
    names
}

pub fn get_file_status(file: MetaFile) -> Status {
    let per = get_percentage(file);
    if per == 100 {
        Status::SEEDING
    } else {
        Status::LEECHING
    }
}

pub fn get_percentage(file: MetaFile) -> usize {
    let myself = PeerConfig::new();
    let buffermap = get_buffermap(myself, &file.hash).unwrap();
    let (_, ones) = count(&buffermap);
    let total = buffermap.len() as f32;
    let result = (ones as f32 / total) * 100.0;
    println!("{}", result);
    result as usize
}

fn get_peers_number(file: MetaFile) -> usize {
    let peers = get_peers_from_file(file.hash);
    peers.len()
}

#[tauri::command]
pub fn get_files_data() -> String {
    let mut data = String::new();
    let files = get_all_files();
    for file in files {
        // Convert the percentage to a string and append it to `data`.
        write!(data, "{}#", file.file_name).unwrap();
        write!(data, "{}#", get_percentage(file.clone())).unwrap();
        write!(data, "{}#", get_peers_number(file.clone())).unwrap();

        // Convert the file status to a string and append it to `data`.
        let status = match get_file_status(file) {
            Status::SEEDING => "1",
            Status::LEECHING => "0",
        };
        write!(data, "{}|", status).unwrap();
    }
    data
}

fn front_get_file_names(input: String) -> Vec<String> {
    let mut valid_files = Vec::new();

    let file_names = input.trim().split_whitespace();

    for file_name in file_names {
        if Path::new(file_name).exists() {
            println!("File {} exists", file_name);
            valid_files.push(file_name.to_string());
        } else {
            println!("File {} does not exist. Skipping.", file_name);
        }
    }
    valid_files
}

#[tauri::command]
pub fn uploadFiles(filenames: String) {
    let peer_config = PeerConfig::new();
    let tracker = TrackerConfig::new();
    let tracker_port = tracker.port;
    let tracker_adress = tracker.address;
    println!("You're in upload");
    let seeded_files = front_get_file_names(filenames); // take the files the user wish to seed
    let seeded_files: Vec<MetaFile> = seeded_files
        .into_iter()
        .map(|file| MetaFile::new(file.to_string()))
        .collect(); // Create vector of Metafiles out of the files name

    let seeded_files2 = seeded_files.clone();
    for seed in seeded_files2 {
        add_seed_file_to_db(seed);
    }
    let seeded_files = seedf(
        seeded_files,
        peer_config.port.to_string(),
        vec!["".to_string()],
    ); // create the message
    println!("Prepared message: {}", seeded_files);
    if let Some(mut stream) = connect(tracker_port, &tracker_adress.to_string()) {
        // connect to the tracker
        send(&mut stream, seeded_files.clone()); // send the message
        println!(
            "Sending to {}:{} : {}",
            stream.peer_addr().unwrap().ip(),
            stream.peer_addr().unwrap().port(),
            seeded_files.clone()
        );
        println!("Message sent waiting for answer");
        let response = receive(&mut stream, 3000); // receive the answer
        println!("Received: {}", response);
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

pub struct DownloadParams {
    pub tcp_size: u32,
    pub pool: Arc<Mutex<Pool>>,
}

#[tauri::command]
pub fn handle_download(state: tauri::State<'_, DownloadParams>, key: String) {
    let params = state.inner();
    let size: u32 = params.tcp_size;
    let mut pool = params.pool.lock().expect("Failed to acquire lock on Pool");
    download_file(key, &mut pool, size);
}

pub fn download_file(filekey: String, pool: &mut Pool, tcp_size: u32) {
    // -> Result<(), Box<dyn std::error::Error>> {
    // The list of downloadable files should be the result of search section
    // todo!();
    println!("You're in download");
    // display files along with their size
    // if two files are name the same user should be able to choose which one to download
    println!("You chose to download: {}", filekey);
    let pool_clone: Pool = pool.clone();
    let tracker = TrackerConfig::new();
    let tracker_adress = tracker.address;
    let tracker_port = tracker.port;
    let result = start_download(
        filekey,
        tracker_port,
        &tracker_adress,
        pool_clone,
        tcp_size as usize,
    );

    match result {
        Ok(task_list) => {
            for task in task_list {
                pool.add_task(task);
            }
        }
        Err(errors) => {
            error!("Could not start download : {}", errors);
        }
    }
}
