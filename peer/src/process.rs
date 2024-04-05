use crate::tasks::{data, getpieces, have, interested, Task};
/// write a data to TCP
impl Task for getpieces {
    // write message
    fn process(&self) {
        let pieces = format!("ok");
        let message = format!("data {} {}\n", self.key, pieces);
    }
    // send message
}
