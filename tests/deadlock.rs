use std::sync::{Arc, Mutex};
use std::thread;

use sysx::time::time::sleep;
use sysx::utils::deadlock::deadlock_detection_thread;

#[test]
fn test_deadlock_detection() {
    let mutex1 = Arc::new(Mutex::new(1));
    let mutex2 = Arc::new(Mutex::new(2));

    thread::spawn(deadlock_detection_thread);

    let _ = thread::spawn(move || {
        let _a = mutex1.lock().unwrap();
        sleep(100);
        let _b = mutex2.lock().unwrap();
    });

    sleep(2000);
}
