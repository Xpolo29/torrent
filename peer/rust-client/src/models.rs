// src/models.rs
use config::{Config, File, ConfigError};
use std::path::PathBuf;
pub struct FileProp {
    pub file_name: String,
    pub length: u64,
    pub piece_size: u64,
    pub hash: String,
}

pub struct PeerConfig {
    pub tracker_address: String,
    pub tracker_port: i64,
    pub shared_files: Vec<String>,
}

pub fn handle_config() -> Result<PeerConfig, ConfigError> {
    let config_path = PathBuf::from("../../config.ini");

    let settings = Config::builder()
        .add_source(File::with_name(config_path.to_str().unwrap()))
        .build()?;

    let tracker_address: String = settings.get_string("tracker-address")?;
    let tracker_port: i64 = settings.get_int("tracker-port")?;

    Ok(PeerConfig {
        tracker_address,
        tracker_port,
        shared_files: vec!["test_taille.txt".to_string()],
    })
}

pub enum Answer {
    Ok,
    List(Vec<FileProp>),
}

pub trait ExpectedAnswer {
    fn parse_answer(&self, answer: String) -> Result<Answer, &'static str>;
}

pub struct ExpectOk;
pub struct ExpectList;
