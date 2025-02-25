/// Periodically checks for deadlocks every 10 seconds using the parking_lot library.
/// If any deadlocks are detected, it prints the number of deadlocks and details for each
/// deadlocked thread, including the thread ID and backtrace. This function runs indefinitely.
/// Example:
/// ```rust
/// use sysx::deadlock::deadlock_detection_thread;
/// use std::thread;
///
/// fn main() {
///     thread::spawn(deadlock_detection_thread);
///     // You'r code
/// }
/// ```
pub fn deadlock_detection_thread() {
    loop {
        let _out = crate::time::safe_sleep("10s");
        let deadlocks = parking_lot::deadlock::check_deadlock();
        if deadlocks.is_empty() {
            continue;
        }

        println!("{} deadlocks detected", deadlocks.len());
        for (i, threads) in deadlocks.iter().enumerate() {
            println!("Deadlock #{i}");
            for t in threads {
                println!("Thread Id {:#?}", t.thread_id());
                println!("{:#?}", t.backtrace());
            }
        }
    }
}
