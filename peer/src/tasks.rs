use hashbrown::HashMap;
use std::net::TcpStream;
use crate::data::PeerConfig;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use crate::threads::Pool;

/// task struct, which is the parent class
pub trait Task: Send {
    fn process(&mut self);
}

/// empty task
pub struct EmptyTask {
    pub stream: Option<TcpStream>,
}

/// To be processed task, to get listener free
pub struct ToBeProcessed {
    pub tasklist: Arc<Mutex<VecDeque<Box<dyn Task + Send>>>>,
    pub stream: TcpStream,
}

/// Receieved via TCP getpieces and return a data request to be send
pub struct Getpieces {
    pub key: String,
    pub pieces: Vec<usize>,
    pub stream: Option<TcpStream>,
}

/// Receieved via TCP interested and return a have request to be send
pub struct Interested {
    pub key: String,
    pub stream: Option<TcpStream>,
}

/// Receieved via TCP have and return a interested request to be send
pub struct Have {
    pub key: String,
    pub buffermap: Vec<u8>,
    pub stream: Option<TcpStream>,
}

pub struct Data {
    pub key: String,
    pub pieces: Vec<(usize, Vec<u8>)>,
    pub stream: Option<TcpStream>,
}
/// send a get_piece, recieve the data and write it
pub struct DataWrite {
    pub peer: PeerConfig,
    pub file_key: String,
    pub pool: Pool, 
}

#[derive(Debug)]
pub struct Peer {
    pub hash: String,
    pub config: PeerConfig,
    pub pool: Pool,
}
