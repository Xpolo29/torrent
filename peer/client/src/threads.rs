use std::sync::Arc;
use tokio::sync::Mutex;
use rand::Rng;
use tokio::task;
use tokio::sync::mpsc;
use rayon::prelude::*; // for thread pool
use std::thread;
use std::thread::ThreadId;

pub struct Pool {
    stack: Arc<Mutex<Vec<Task>>>,
    pool: rayon::ThreadPool,
    tx : tokio::sync::mpsc::Sender<Task>,
    rx: tokio::sync::mpsc::Receiver<Task>
}

pub struct Task {
    value: i32,
}

impl Task {
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}

impl Pool {
    pub fn new(num_threads : usize, task_len: usize) -> Self {
        let stack = Arc::new(Mutex::new(Vec::new()));
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build()
            .unwrap();

        let (tx, mut rx) = mpsc::channel::<Task>(task_len);
        Self { stack, pool , tx, rx}
    }

    pub fn execute(&self){
        let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

        rt.block_on(self.execute_async());
    }

    async fn process_task(&self, task: Task, thread_id: ThreadId) {
        println!("Thread {:?} processing task: {}", thread_id, task.value);
    }

    async fn execute_async(&self) {
        loop {
            if let Some(task) = self.rx.recv().await {
                self.pool.spawn(move || {
                    self.process_task(task);
                });
            } else {
                break; // Exit the loop if the channel is closed
            }
        }
    }


    async fn add_task_async(&self, task: Task) {
        let mut stack = self.stack.lock().await;
        stack.push(task);
    }

    pub fn add_task(&self, task: Task){
        let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

        rt.block_on(self.add_task_async(task));
    }

    fn get_id(&self) -> usize {
        let mut rng = rand::thread_rng();
        let random_number: usize = rng.gen_range(0..100);
        random_number
    }
}
