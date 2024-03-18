// src/network_config.rs
use config::{Config, ConfigError, File};
use std::path::Path;

pub struct FileProp {
    pub file_name: String,
    pub length: u64,
    pub piece_size: u64,
    pub hash: String,
}
/// This config is used to store the port, IP, and shared files of **EACH** peer
/// and not the port and IP used to communicate with the tracker
pub struct PeerConfig {
    pub port: u16,
    pub ip: String,
    pub files: Vec<FileProp>,
}
impl PeerConfig {
    pub fn new() -> Self {
        Self {
            port: 8080,
            ip: "127.0.0.1".to_string(),
            files: Vec::new(),
        }
    }
}
/// This config is used to store the port and IP used to communicate with the tracker
/// its fields can be retrive from config.ini and maybe edited by the user
pub struct TrackerConfig {
    pub port: u16,
    pub ip: String,
}
impl TrackerConfig {
    pub fn new() -> Self {
        Self {
            port: 7878,
            ip: "127.0.0.1".to_string(),
        }
    }
    pub fn from_config() -> Result<Self, ConfigError> {
        let config_path = Path::new("../../../config.ini");
        if !config_path.exists() {
            println!("No config file found, using default values");
            return Ok(Self::new());
        }

        let s = Config::builder()
            .add_source(File::with_name("../../../config.ini").required(false))
            .build()?;

        let port: u16 = s.get("tracker-port")?;
        let ip: String = s.get("tracker-address")?;

        Ok(Self { port, ip })
    }
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
