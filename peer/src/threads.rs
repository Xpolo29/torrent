use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use crate::tasks::Task; 
use std::collections::VecDeque;


//gloval var, used to stop threads
static mut RUNNING: bool = true;


//pool struct
pub struct Pool {
    //tasklist : Arc<Mutex<Vec<Task>>>,
    tasklist : Arc<Mutex<VecDeque<Task>>>,
    thread_pool : Vec<std::thread::JoinHandle<i32>>,
}



impl Pool {
    //Pool::pool.new(NB_THREADS)
    pub fn new(size : i32) -> Pool{

        let mut thread_pool = Vec::new();
        let tasklist : Arc<Mutex<VecDeque<Task>>> = Arc::new(Mutex::new(VecDeque::new()));

        for i in 0..size  {
            let clone = Arc::clone(&tasklist);
            let handle = thread::spawn(move || {
                let res : i32 = 0;
                let id : i32 = i;

                println!("thread {} started", id);

                unsafe{
                    let mut option : Option<Task>;
                    let mut len : usize;
                    while RUNNING {
                        {
                            let mut data = clone.lock().unwrap();
                            option = data.pop_front();
                            len = data.len();
                        }
                        if len > 0 {
                            match option {
                                Some(task) => {task.process(id)}
                                None => {}
                            }
                        } else {
                            thread::sleep(Duration::from_millis(10));
                        }
                    }
                }
            
                res
            });
            thread_pool.push(handle);
        }

        Pool {tasklist, thread_pool}
    }

    //add task to pool
    pub fn add_task(&mut self, t : Task) -> () {
            let mut data = self.tasklist.lock().unwrap();
            data.push_back(t);
    }

    //join threads (wait for them to die)
    fn join(self){
        for thread in self.thread_pool {
            thread.join().unwrap();
        }
    }

    //ask for a clean exit, finish all pending tasks first
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


