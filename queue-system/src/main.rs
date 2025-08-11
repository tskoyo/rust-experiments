use std::{
    os::unix::thread::JoinHandleExt,
    sync::{Arc, Mutex, mpsc},
    thread,
    time::Duration,
};

struct ThreadPool {
    sender: Option<mpsc::Sender<Message>>,
    workers: Vec<thread::JoinHandle<()>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool {
    fn new(size: usize) -> Self {
        let (sender, receiver) = mpsc::channel::<Message>();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for _ in 0..size {
            let r = Arc::clone(&receiver);
            workers.push(thread::spawn(move || {
                loop {
                    let message = r.lock().unwrap().recv().expect("Failed to receive message");
                    match message {
                        Message::NewJob(job) => job(),
                        Message::Terminate => break,
                    }
                }
            }));
        }

        ThreadPool {
            sender: Some(sender),
            workers,
        }
    }

    fn execute<F>(&mut self, task: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender
            .as_ref()
            .unwrap()
            .send(Message::NewJob(Box::new(task)))
            .unwrap();
        println!("Task added to the pool");
    }

    fn shutdown(self) {
        println!("Sending termination messages to workers");
        for _ in &self.workers {
            self.sender
                .as_ref()
                .unwrap()
                .send(Message::Terminate)
                .unwrap();
        }

        for worker in self.workers {
            if let Err(e) = worker.join() {
                eprintln!("Worker thread failed to join: {:?}", e);
            }
        }
        println!("All worker threads have been terminated");
    }
}

fn main() {
    let mut pool = ThreadPool::new(2);

    for i in 0..6 {
        let task = move || {
            println!("Task {} is running", i);
            thread::sleep(Duration::from_secs(1));
            println!("Task {} completed", i);
        };
        pool.execute(task);
    }

    pool.shutdown();
}
