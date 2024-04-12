use hashbrown::HashMap;
use num_traits::Saturating;
use std::net::TcpStream;

use crate::data::PeerConfig;

/// task struct, which is the parent class
pub trait Task: Send {
    fn process(&mut self);
}

/// empty task
pub struct EmptyTask {
    pub stream: Option<TcpStream>,
}

impl Task for EmptyTask {
    fn process(&mut self) {
        println!("This is an empty task");
    }
}

/// receieved via TCP getpieces and return a data request to be send
pub struct Getpieces {
    pub key: String,
    pub pieces: Vec<u32>, // should be usize?
    pub stream: Option<TcpStream>,
}
/// receieved via TCP interested and return a have request to be send
pub struct Interested {
    pub key: String,
    pub stream: Option<TcpStream>,
}
/// receieved via TCP have and return a interested request to be send
pub struct Have {
    pub key: String,
    pub buffermap: Vec<u8>,
    pub stream: Option<TcpStream>,
}
pub struct Data {
    pub key: String,
    pub pieces: HashMap<u32, Vec<u8>>,
    pub stream: Option<TcpStream>,
}

pub struct Peers {
    pub key: String,
    pub peers: PeerConfig,
}
