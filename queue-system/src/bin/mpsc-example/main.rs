use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (sender, received) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec!["hello", "from", "thread"];

        for val in vals {
            sender.send(val).unwrap();
            thread::sleep(Duration::from_millis(1500));
        }
    });

    for r in received {
        println!("Got: {}", r);
    }
}
