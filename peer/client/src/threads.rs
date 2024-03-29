use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};
use rand::Rng;

pub struct Pool {
    stack: Arc<Mutex<Vec<Task>>>,
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
    pub fn new() -> Self {
        let stack = Arc::new(Mutex::new(Vec::new()));
        Self { stack }
    }

    pub async fn execute(&self) {
        let stack_clone = Arc::clone(&self.stack);
        let (tx, mut rx) = mpsc::channel(10);

        for _ in 0..10 {
            let stack_clone = Arc::clone(&stack_clone);
            let tx = tx.clone();

            tokio::spawn(async move {
                let mut stack = stack_clone.lock().await;
                while let Some(task) = stack.pop() {
                    println!("Processing {}", task.value);
                }
                tx.send(()).await.unwrap();
            });
        }

        drop(tx);

        while let Some(_) = rx.recv().await {
            // Wait for all tasks to complete.
        }
    }

    pub async fn add_task(&self, task: Task) {
        let mut stack = self.stack.lock().await;
        stack.push(task);
    }

    fn get_id(&self) -> usize {
        let mut rng = rand::thread_rng();
        let random_number: usize = rng.gen_range(0..100);
        random_number
    }
}
