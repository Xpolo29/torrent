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
    //pub tasklist: Arc<Mutex<VecDeque<Box<dyn Task + Send>>>>,
    pub pool: Pool,
    pub stream: TcpStream,
}

/// Receieved via TCP getpieces and return a data request to be send
pub struct Getpieces {
    pub key: String,
    pub chunk_size: usize,
    pub pieces: Vec<usize>,
    pub stream: Option<TcpStream>,
    pub pool: Pool,
    pub retry: usize,
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

impl Clone for Have {
    fn clone(&self) -> Self {
        Have {
            key: self.key.clone(),
            buffermap: self.buffermap.clone(),
            stream: None,
        }
    }
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
    pub nb_pieces: usize,
    pub pool: Pool, 
    pub stream: Option<TcpStream>,
}

impl Clone for DataWrite {
    fn clone(&self) -> Self {
        DataWrite {
            peer: self.peer.clone(),
            file_key: self.file_key.clone(),
            nb_pieces: self.nb_pieces,
            pool: self.pool.clone(),
            stream: None,
        }
    }
}


#[derive(Debug)]
pub struct Peer {
    pub hash: String,
    pub length_tcp: usize,
    pub config: PeerConfig,
    pub pool: Pool,
}
