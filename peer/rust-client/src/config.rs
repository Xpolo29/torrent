// src/config.rs
pub struct FileProp {
    pub file_name: String,
    pub length: u64,
    pub piece_size: u64,
    pub hash: String,
}

pub struct Config {
    pub port: u16,
    pub files: Vec<FileProp>,
}
