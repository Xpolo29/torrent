use crate::data::PeerConfig;
use crate::db::get_buffermap;
use crate::tasks::{Data, Getpieces, Have, Interested, Task};
use log::error;
impl Task for Interested {
    fn process(&self) {
        println!("This is an interested request");
    }
}
/// write a data to TCP
impl Task for Getpieces {
    fn process(&self) {
        let peer_ip = self
            .stream
            .as_ref()
            .unwrap()
            .peer_addr()
            .unwrap()
            .ip()
            .to_string();
        let peer_port = self.stream.as_ref().unwrap().peer_addr().unwrap().port();
        let peer_config = PeerConfig::new(peer_ip, peer_port);
        let buffermap = get_buffermap(peer_config, &self.key);
        match buffermap {
            Some(buffermap) => {
                let pieces: Vec<String>;
            }
            None => {
                error!("No buffermap found for key {}", self.key);
            }
        }
    }
}

impl Task for Data {
    fn process(&self){}
}

impl Task for Have {

    fn process(&self){}
}

