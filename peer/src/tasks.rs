use hashbrown::HashMap;

/// task struct, which is the parent class
pub trait Task {
    fn process(&self);
}

/// receieved via TCP getpieces and return a data request to be send
pub struct getpieces {
    pub key: String,
    pub pieces: Vec<u32>,
}
/// receieved via TCP interested and return a have request to be send
pub struct interested {
    pub key: String,
}
/// receieved via TCP have and return a interested request to be send
pub struct have {
    pub key: String,
    pub buffermap: Vec<u8>,
}
pub struct data {
    pub key: String,
    pub pieces: HashMap<u32, Vec<u8>>,
}
