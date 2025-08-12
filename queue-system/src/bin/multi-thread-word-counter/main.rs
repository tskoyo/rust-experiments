use std::{sync::mpsc, thread};
use thread::JoinHandle;

fn main() {
    let texts = vec![
        "hello world from rust",
        "this is a message passing example",
        "channels are great for thread communication",
        "rust concurrency is safe and fast",
    ];

    let (producer, consumer) = mpsc::channel();
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    for i in texts {
        let sender = producer.clone();
        let handle = thread::spawn(move || {
            let word_number = i.split_whitespace().count();
            sender.send(word_number).unwrap();
        });
        handles.push(handle);
    }

    drop(producer);

    for received in consumer {
        println!("Number of words: {}", received);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
