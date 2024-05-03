use crate::com::{connect, send, update, receive};
use crate::data::{PeerConfig, TrackerConfig};
use crate::parser::parse_request;
use crate::tasks::Task;
use log::*;
use std::collections::VecDeque;
use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::{thread, fmt};
use std::time::Duration;
use crate::tasks::{EmptyTask, ToBeProcessed};

// for UPnP
use easy_upnp::{add_ports, delete_ports, Ipv4Cidr, PortMappingProtocol, UpnpConfig};
use std::error::Error;


// gloval var, used to stop threads
static mut RUNNING: bool = true;

// clean exit, idk how to do it 
static mut PORT: u16 = 0;

//pool struct
pub struct Pool {
    //tasklist : Arc<Mutex<Vec<Task>>>,
    tasklist: Arc<Mutex<VecDeque<Box<dyn Task + Send>>>>,
    thread_pool: Arc<Mutex<VecDeque<std::thread::JoinHandle<i32>>>>,
}

impl Clone for Pool {
    fn clone(&self) -> Self {
        Pool {
            tasklist: self.tasklist.clone(),
            thread_pool: self.thread_pool.clone(),
        }
    }
}

impl fmt::Debug for Pool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .finish()
    }
}

impl Pool {
    //Pool::pool.new(NB_THREADS)
    pub fn new(size: i32) -> Pool {
        let thread_pool = Arc::new(Mutex::new(VecDeque::new()));
        let tasklist: Arc<Mutex<VecDeque<Box<dyn Task + Send>>>> =
            Arc::new(Mutex::new(VecDeque::new()));

        for i in 0..size {
            let clone = Arc::clone(&tasklist);
            let handle = thread::spawn(move || {
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
            });
            {thread_pool.lock().unwrap().push_front(handle);}
        }

        Pool {
            tasklist,
            thread_pool,
        }
    }




    pub fn start_listening(&mut self, pc: PeerConfig) {

        // listen to port
        let add = format!("{}:{}", pc.address, pc.port);
        debug!("Listening on {}", add);
        let door = TcpListener::bind(add).unwrap();
        let thread_pool_clone = self.thread_pool.clone();

        let tasklist_clone = self.tasklist.clone();

        // try to bind router port to us
        // try_upnp(pc.port);

        let lithread = thread::spawn(move || {

            while unsafe { RUNNING } {
                for con in door.incoming() {
                    match con {
                        Ok(stream) => {
                            debug!("incoming from {}", stream.peer_addr().unwrap());

                            let stream = stream.try_clone().unwrap();

                            let tasklist = tasklist_clone.clone();

                            let tbp: ToBeProcessed = ToBeProcessed {tasklist, stream}; 
                            {
                                tasklist_clone.lock().unwrap().push_front(Box::new(tbp));
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

    /// start update thread
    pub fn start_update(&mut self, tc: TrackerConfig, period: i32) {
        let upthread = thread::spawn(move || {
            unsafe {
                while RUNNING {
                    let msg: String = update();
                    if let Some(mut stream) = connect(tc.port, tc.address.as_str()) {
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

    pub fn add_task(&mut self, task: Box<dyn Task + Send>){// + 'static>) {
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
    pub fn handle_client(tasklist: Arc<Mutex<VecDeque<Box<dyn Task + Send>>>>, mut stream: TcpStream) {
        /*
        let mut reader = BufReader::new(&mut stream);
        let mut buff: Vec<u8> = Vec::new();
        let bytes_read = reader.read_until(b'\n', &mut buff).unwrap();

        if bytes_read > 0 {
        */
            //let msg: String = String::from_utf8_lossy(&buff).into_owned();
            let msg = receive(&mut stream);
            info!("Received msg {}", msg);
            let task : Box<(dyn Task + Send + 'static)> = parse_request(msg, Some(stream));
            let mut data = tasklist.lock().unwrap();
            data.push_back(task);
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
        let t1: EmptyTask = EmptyTask { stream: None}; 
        let t2: EmptyTask = EmptyTask { stream: None}; 
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

        let t1: EmptyTask = EmptyTask { stream: None}; 
        let t2: EmptyTask = EmptyTask { stream: None}; 
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

fn get_upnp_config(port: u16) -> [UpnpConfig; 1]{
    let config: UpnpConfig = UpnpConfig {
        address: None,
        port: port,
        protocol: PortMappingProtocol::TCP,
        duration: 3600,
        comment: "peer".to_string(),
    };
    [config]
}

fn try_upnp(port: u16){
    unsafe{
        PORT = port;
    }
    for res in add_ports(get_upnp_config(port)){
        if res.is_err() {
            error!("Failed to bind UPnP, outside connection will be refused")
        }
    }
}

fn close_upnp(){
    let port: u16;
    unsafe{
        port = PORT;
    }
    for res in delete_ports(get_upnp_config(port)){
        if res.is_err() {
            error!("Failed to unbind UPnP")
        }
    }

}
