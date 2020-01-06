use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use rand::Rng;

fn main() {
//    simple_single_threaded_example();
    multi_threaded_example();
}

fn simple_single_threaded_example() {
    let m: Mutex<i32> = Mutex::new(5);
    println!("before: m = {:?}", m);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("after: m = {:?}", m);
}

fn multi_threaded_example() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for thread_id in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let wait = rand::thread_rng().gen_range(50, 500);
            thread::sleep(Duration::from_millis(wait));
            println!("Thread {} increments counter after waiting {} milliseconds", thread_id, wait);
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
