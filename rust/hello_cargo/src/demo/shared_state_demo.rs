use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;

pub fn run() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            // drop(num); // 可以用drop来释放锁
            thread::sleep(Duration::from_secs(1));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}