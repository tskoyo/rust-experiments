use std::{sync::mpsc, thread, time::Duration};
fn main() {
    let (tx1, rx1) = mpsc::channel::<i32>(); // producer -> stage 1
    let (tx2, rx2) = mpsc::sync_channel::<i32>(2); // stage 1 -> stage 2
    let (tx3, rx3) = mpsc::channel::<String>(); // stage 2 -> Main

    let mut handles = Vec::new();

    let handle_1 = thread::spawn(move || {
        for data in 1..=150 {
            tx1.send(data).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
        println!("Producer: done");
    });

    // Stage 1 thread
    let handle_2 = thread::spawn(move || {
        for data in rx1 {
            let square = data * data;
            println!(
                "Stage 1: computed {}² = {}, sending to Stage 2",
                data, square
            );
            tx2.send(square).unwrap();
        }

        println!("Stage 1: done");
    });

    // Stage 2 thread
    let handle_3 = thread::spawn(move || {
        for data in rx2 {
            println!("Stage 2: received {}, processing...", data);
            thread::sleep(Duration::from_secs(5)); // simulate slow stage
            let result = String::from(format!("Squared number: {}", data));
            tx3.send(result).unwrap();
        }
    });

    handles.push(handle_1);
    handles.push(handle_2);
    handles.push(handle_3);

    for data in rx3 {
        println!("{}", data);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
