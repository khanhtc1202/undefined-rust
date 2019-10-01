use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn run() {
    // one_vs_one();
    // one_vs_one_multi();
    // multi_vs_one();
    one_vs_one_multi_with_rc();
}

#[allow(dead_code)]
fn one_vs_one() {
    // `mpsc` a.k.a multiple producer, single consumer
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap(); // sent the val to the stream

        // we can not do as below since the val be moved
        // println!("val is {}", val);
    });

    // recv block the current thread execution until it receive data from stream
    // which map with rx - receiver end.
    let received = rx.recv().unwrap();
    // and other method is `try_recv` which will try to get message from stream immediately,
    // not block the execution of current thread. Can use with loop to try pull message intervally
    println!("Got: {}", received);
}

#[allow(dead_code)]
fn one_vs_one_multi() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

#[allow(dead_code)]
fn multi_vs_one() {
    let (tx, rx) = mpsc::channel();

    // make a clone of transmitter
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

use std::sync::Arc;

fn one_vs_one_multi_with_rc() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            Arc::new(String::from("hello")),
            Arc::new(String::from("world")),
        ];

        vals.iter().for_each(|val| {
            tx.send(val.clone()).unwrap();
            thread::sleep(Duration::from_secs(1));
        });
    });

    rx.iter().for_each(|mess| {
        println!("Got: {}", mess);
    });
}