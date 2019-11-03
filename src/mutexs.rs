use std::sync::Mutex;

pub fn run() {
//    simple();
//    another_simple();
    multi_thread_mutex();
}

#[allow(dead_code)]
fn simple() {
    let m = Mutex::new(5);

    {
        /// To access the data inside the mutex, we use the lock method to acquire the lock.
        ///
        /// The call to lock would fail if another thread holding the lock panicked. => need unwrap()
        ///
        /// The call to lock returns a smart pointer called `MutexGuard`, wrapped in a `LockResult`
        /// that we handled with the call to unwrap.
        ///
        /// after this line, we going out of inner scope and ref `num` will be drop => return the lock to main thread
        /// using inner scope, we don’t risk forgetting to release the lock and blocking the mutex from being used by other threads
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m); // expect 6
}

#[allow(dead_code)]
fn another_simple() {
    let m = Mutex::new(5);
    let mut num = m.lock().unwrap();
    *num = 6;
    println!("m = {:?}", m); // expect Mutex { data: <locked> } since lock still be hold by num, not m
    drop(num);
    println!("m = {:?}", m); // expect 6 since drop(num) free the lock and it turn back to the main thread
}

use std::thread;
use std::sync::Arc;

fn multi_thread_mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = counter.clone(); // alternate: Arc::clone(&counter);
        let handle = thread::spawn(move || {
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
