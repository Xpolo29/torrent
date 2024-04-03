use std::thread::sleep;
use std::time::Duration; 

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
