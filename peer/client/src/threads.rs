use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

static mut RUNNING: bool = true;

pub struct Pool {
    tasklist : Arc<Mutex<Vec<Task>>>,
    thread_pool : Vec<std::thread::JoinHandle<i32>>,
}

pub struct Task {
    value : i32,
}

impl Pool {
    pub fn new(size : i32) -> Pool{

        let mut thread_pool = Vec::new();
        let tasklist : Arc<Mutex<Vec<Task>>> = Arc::new(Mutex::new(Vec::new()));

        for i in 0..size  {
            let clone = Arc::clone(&tasklist);
            let handle = thread::spawn(move || {
                let res : i32 = 0;
                let id : i32 = i;

                println!("thread {} started", id);

                unsafe{
                    let mut option : Option<Task>;
                    while RUNNING {
                        {
                            let mut data = clone.lock().unwrap();
                            option = data.pop();
                        }
                        match option {
                            Some(task) => {task.process(id)}
                            None => {}
                        }
                    }
                }
            
                res
            });
            thread_pool.push(handle);
        }

        Pool {tasklist, thread_pool}
    }

    pub fn add_task(&mut self, t : Task) -> () {
            let mut data = self.tasklist.lock().unwrap();
            data.push(t);
    }

    fn join(self){
        for thread in self.thread_pool {
            thread.join().unwrap();
        }
    }

    pub fn drop(self){

        println!("Requested threads stop");

        loop {
            let len : usize;
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

        unsafe{
            RUNNING = false;
        }
        self.join();
        println!("All threads have been stopped");
    }
}

impl Task {
    pub fn new(value : i32) -> Task {
        Task { value }
    }

    pub fn process(self, thread_id : i32){
        println!("{} is processing {}", thread_id, self.value);
        thread::sleep(Duration::from_millis(100));
    }
}
