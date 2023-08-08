use std::sync::{Arc, Mutex};
use std::{
    sync::mpsc,
    thread::{self, JoinHandle},
    time::Duration,
};

fn main() {
    // single_worker();
    // pass_message();
    //pass_message_multi();
    //atmoic_mutex();
}

fn single_worker() {
    let worker = thread::spawn(|| {
        for i in 1..=10 {
            println!("number {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..=5 {
        println!("dogsniff {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    worker.join().unwrap();
}

fn pass_message() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msg: String = String::from("hi");
        tx.send(msg).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn pass_message_multi() {
    let (tx, rx) = mpsc::channel();

    let mut workers: Vec<JoinHandle<()>> = vec![];
    for i in 1..=10 {
        let tx2 = tx.clone();
        let worker = thread::spawn(move || {
            tx2.send(i).unwrap();
        });
        workers.push(worker);
    }

    for _ in workers {
        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }
}

fn atmoic_mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 1..=10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
        println!("count: {}", counter.lock().unwrap());
    }
}
