use crate::data::{get_file_key, MetaFile, PeerConfig};
use crate::db::*;
use std::fmt::Write;

enum Status {
    LEECHING = 0,
    SEEDING = 1,
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
    ones / buffermap.len() * 100
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
} // add #[tauri::command] before every function to be used in javascript
  // functions to be defined
  // get data for dashboard (files, download percentage, peers, leeching status )
  // backend for upload files
  // backend for download (search) files
