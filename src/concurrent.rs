use std::thread;
use std::time::Duration;

pub fn run() {
//    spawn_a_thread();
//    make_prog_wait_until_thread_done();
//    block_main_thread_by_join();
    move_value_to_spawned_thread();
}

fn spawn_a_thread() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn make_prog_wait_until_thread_done() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn block_main_thread_by_join() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn move_value_to_spawned_thread() {
    let v = vec![1,2,3];

    // move keyword force the spawned thread take the ownership of all the value it use from outer scope
    let handle = thread::spawn(move || {
        println!("vector = {:?}", v);
        drop(v); // unnecessary drop since value will be clean up after the thread done
    });

    // since the value `v` been moved to the spawned thread, we no longer able to drop it from main thread
    // below statement will cause compiler err ( caught by borrow checker )
    // drop(v);

    handle.join().unwrap();
}