use std::time::Duration;
use std::thread;


pub fn sleep(time: u64) {
    thread::sleep(Duration::from_millis(time));
}
