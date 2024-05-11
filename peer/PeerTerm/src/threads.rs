use crate::back::store_have_to_db;
use crate::com::{connect, havef, receive, send, updatef};
use crate::data::{MetaFile, PeerConfig, TrackerConfig};
use crate::db::{get_buffermap, get_leeching_files, get_peers_from_file};
use crate::parser::{parse_have_from_have, parse_request};
use crate::tasks::Task;
use crate::tasks::{EmptyTask, Have, ToBeProcessed};
use log::{debug, error, info, trace, warn};
use rayon::prelude::*;
use std::collections::VecDeque;
use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{fmt, thread};

// for UPnP
use easy_upnp::{add_ports, delete_ports, Ipv4Cidr, PortMappingProtocol, UpnpConfig};
use std::error::Error;

// gloval var, used to stop threads
static mut RUNNING: bool = true;

// clean exit, idk how to do it
static mut PORT: u16 = 0;

//pool struct
pub struct Pool {
    tasklist: Arc<Mutex<VecDeque<Box<dyn Task + Send>>>>,
    thread_pool: Arc<Mutex<VecDeque<std::thread::JoinHandle<i32>>>>,
    size: usize,
}

impl Clone for Pool {
    fn clone(&self) -> Self {
        Pool {
            tasklist: self.tasklist.clone(),
            thread_pool: self.thread_pool.clone(),
            size: self.size,
        }
    }
}

impl fmt::Debug for Pool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point").finish()
    }
}

impl Pool {
    //Pool::pool.new(NB_THREADS)
    pub fn new(size: i32) -> Pool {
        let thread_pool = Arc::new(Mutex::new(VecDeque::new()));
        let tasklist: Arc<Mutex<VecDeque<Box<dyn Task + Send>>>> =
            Arc::new(Mutex::new(VecDeque::new()));

        (0..size).into_par_iter().for_each(|i| {
            let clone = Arc::clone(&tasklist);
            let handle = thread::Builder::new()
            //let handle = thread::spawn(move || {
            .name(i.to_string())
            .spawn(move || {
                let res: i32 = 0;
                let id: i32 = i;

                debug!("thread {} started", id);

                unsafe {
                    let mut option: Option<Box<dyn Task + Send>>;
                    let mut len: usize;
                    while RUNNING {
                        {
                            let mut data = clone.lock().unwrap();
                            len = data.len();
                            if len > 0 {
                                option = data.pop_front();
                            } else {
                                option = None;
                            }
                        }
                        if len > 0 {
                            trace!("Thread {} is processing a task", id);
                            match option {
                                Some(mut task) => task.process(),
                                None => {}
                            }
                            trace!("Thread {} has finished processing a task", id);
                        } else {
                            thread::sleep(Duration::from_millis(10));
                        }
                    }
                }

                res
            }).unwrap();
            {
                thread_pool.lock().unwrap().push_front(handle);
            }
        });

        Pool {
            tasklist,
            thread_pool,
            size: size as usize,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn start_listening(&mut self, pc: PeerConfig) {
        // listen to port
        let add = format!("{}:{}", pc.address, pc.port);
        debug!("Listening on {}", add);
        let door = TcpListener::bind(add).unwrap();
        let thread_pool_clone = self.thread_pool.clone();

        //let tasklist_clone = self.tasklist.clone();

        // try to bind router port to us
        // try_upnp(pc.port);
        let pool_clone: Pool = self.clone();

        let lithread = thread::spawn(move || {
            while unsafe { RUNNING } {
                for con in door.incoming() {
                    match con {
                        Ok(stream) => {
                            debug!("incoming from {}", stream.peer_addr().unwrap());

                            let stream = stream.try_clone().unwrap();

                            let tbp: ToBeProcessed = ToBeProcessed {
                                pool: pool_clone.clone(),
                                stream,
                            };
                            {
                                //tasklist_clone.lock().unwrap().push_front(Box::new(tbp));
                                pool_clone.clone().add_task(Box::new(tbp));
                            }
                        }
                        Err(e) => {
                            error!("{}", e);
                        }
                    }
                }
            }
            0
        });

        {
            thread_pool_clone.lock().unwrap().push_front(lithread);
        }
    }

    /// start have thread
    pub fn start_have(&mut self, period: i32) {
        let havethread = thread::spawn(move || {
            unsafe {
                while RUNNING {
                    // send them a have request

                    let main_config: PeerConfig = PeerConfig::new();
                    let leeching_files: Vec<MetaFile> = get_leeching_files();

                    // foreach leeching file
                    leeching_files.par_iter().for_each(|file| {
                        //for file in leeching_files {
                        let peers = get_peers_from_file(file.hash.clone());
                        let buffmap: Vec<u8> =
                            get_buffermap(main_config.clone(), &file.hash.clone()).unwrap();

                        // get list of peers
                        for peer in peers {
                            if peer.address.clone() == main_config.address.clone()
                                && peer.port == main_config.port
                            {
                                continue;
                            }
                            let ip: String = peer.address.clone();
                            let port: u16 = peer.port;
                            let stream_option: Option<TcpStream> = connect(port, &ip);
                            match stream_option {
                                Some(mut stream) => {
                                    let msg: String = havef(file.hash.clone(), buffmap.clone());
                                    info!("Sending have to {}:{}", ip, port);
                                    send(&mut stream, msg);
                                    let answer: String = receive(&mut stream, 3000);

                                    let have_option: Option<Have> = parse_have_from_have(answer);

                                    // and update their buffermap
                                    match have_option {
                                        Some(have) => store_have_to_db(peer, have),
                                        None => warn!("Received wrong have answer"),
                                    }

                                }
                                None => warn!("Could not send have to {}:{}", ip, port),
                            }
                        }
                    });
                    thread::sleep(Duration::from_secs(period as u64));
                }
            }
            0
        });
        {
            self.thread_pool.lock().unwrap().push_front(havethread);
        }
    }

    /// start update thread
    pub fn start_update(&mut self, tc: TrackerConfig, period: i32) {
        let upthread = thread::spawn(move || {
            unsafe {
                while RUNNING {
                    let msg: String = updatef();
                    if let Some(mut stream) = connect(tc.port, tc.address.as_str()) {
                        info!("Sending update to tracker");
                        send(&mut stream, msg);
                    } 
                    thread::sleep(Duration::from_secs(period as u64));
                }
            }
            0
        });
        {
            self.thread_pool.lock().unwrap().push_front(upthread);
        }
    }

    pub fn add_task(&mut self, task: Box<dyn Task + Send>) {
        // + 'static>) {
        let mut data = self.tasklist.lock().unwrap();
        data.push_back(task);
    }

    //join threads (wait for them to die)
    fn join(self) {
        let mut len: usize = 1;
        let mut thread: std::thread::JoinHandle<i32>;

        while len > 0 {
            {
                let mut data = self.thread_pool.lock().unwrap();
                thread = data.pop_front().unwrap();
                len = data.len();
            }
            thread.join().unwrap();
        }
    }

    //ask for a clean exit, finish all pending tasks first
    pub fn drop(self) {
        debug!("Requested threads stop");

        loop {
            let len: usize;
            {
                let data = self.tasklist.lock().unwrap();
                len = data.len();
            }
            if len > 0 {
                thread::sleep(Duration::from_millis(100));
            } else {
                break;
            }
        }

        unsafe {
            RUNNING = false;
        }
        self.join();
        info!("All threads have been stopped");
        // close_upnp();
    }
}

/// used by listening thread
pub fn handle_client(mut pool: Pool, mut stream: TcpStream) {
    /*
    let mut reader = BufReader::new(&mut stream);
    let mut buff: Vec<u8> = Vec::new();
    let bytes_read = reader.read_until(b'\n', &mut buff).unwrap();

    if bytes_read > 0 {
    */
    //let msg: String = String::from_utf8_lossy(&buff).into_owned();
    let peer = stream.peer_addr().unwrap();
    let ip = peer.ip();
    let port = peer.port();
    info!("Incoming connection from {}:{}", ip, port);
    let msg: String = receive(&mut stream, 3000);
    debug!("Received msg {}", msg.chars().take(128).collect::<String>());
    let task: Box<(dyn Task + Send + 'static)> = parse_request(msg, Some(stream), pool.clone());
    pool.add_task(task);
    /*} else {
        error!("Connection close by {:?}", stream.peer_addr());
    }
    */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_threads_empty() {
        // Set up the test
        let mut pool: Pool = Pool::new(0);
        let mut len: i32;
        {
            let pool_clone = pool.clone();
            let tasklist_clone = pool_clone.tasklist.clone();
            let data = tasklist_clone.lock().unwrap();
            len = data.len() as i32;
        }
        assert_eq!(len, 0);
        let t1: EmptyTask = EmptyTask { stream: None };
        let t2: EmptyTask = EmptyTask { stream: None };
        pool.add_task(Box::new(t1));
        pool.add_task(Box::new(t2));
        {
            let pool_clone = pool.clone();
            let tasklist_clone = pool_clone.tasklist.clone();
            let data = tasklist_clone.lock().unwrap();
            len = data.len() as i32;
        }
        assert_eq!(len, 2);
    }
    #[test]
    fn test_threads() {
        // Set up the test
        let mut pool: Pool = Pool::new(2);
        let mut len: i32;
        {
            let pool_clone = pool.clone();
            let tasklist_clone = pool_clone.tasklist.clone();
            let data = tasklist_clone.lock().unwrap();
            len = data.len() as i32;
        }
        assert_eq!(len, 0);

        let t1: EmptyTask = EmptyTask { stream: None };
        let t2: EmptyTask = EmptyTask { stream: None };
        pool.add_task(Box::new(t1));
        pool.add_task(Box::new(t2));
        std::thread::sleep(Duration::from_millis(200));
        {
            let pool_clone = pool.clone();
            let tasklist_clone = pool_clone.tasklist.clone();
            let data = tasklist_clone.lock().unwrap();
            len = data.len() as i32;
        }
        assert_eq!(len, 0);
        pool.drop();
    }
}

fn get_upnp_config(port: u16) -> [UpnpConfig; 1] {
    let config: UpnpConfig = UpnpConfig {
        address: None,
        port: port,
        protocol: PortMappingProtocol::TCP,
        duration: 3600,
        comment: "peer".to_string(),
    };
    [config]
}

fn try_upnp(port: u16) {
    unsafe {
        PORT = port;
    }
    for res in add_ports(get_upnp_config(port)) {
        if res.is_err() {
            error!("Failed to bind UPnP, outside connection will be refused")
        }
    }
}

fn close_upnp() {
    let port: u16;
    unsafe {
        port = PORT;
    }
    for res in delete_ports(get_upnp_config(port)) {
        if res.is_err() {
            error!("Failed to unbind UPnP")
        }
    }
}
