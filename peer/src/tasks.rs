use std::thread::sleep;
use std::time::Duration; 
use hashbrown::HashMap;
//task struct
pub struct Task {
    value : i32,
}

impl Task {
    pub fn new(value : i32) -> Task {
        Task { value }
    }

    pub fn process(self, thread_id : i32){
        println!("{} is processing {}", thread_id, self.value);
        sleep(Duration::from_millis(100));
    }
}

trait TaskTraitResponse {
    fn process(self, thread_id : i32);
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
    // TODO takes pieces size from config
    pub pieces: HashMap<u32, Vec<u8>>,
}

