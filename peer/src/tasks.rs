use hashbrown::HashMap;

/// task struct, which is the parent class
pub trait Task: Send {

    fn process(&self);

    fn new() {
        empty::new()
    }

}

/// empty task
pub struct empty;

impl Task for empty {

    fn process(&self){
        println!("This is an empty task");
    }

    fn new(){
        empty
    }
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
