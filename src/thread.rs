use std::sync::{Arc, Mutex, mpsc};
use std::thread::{self, JoinHandle};

// `Send` transfer between threads
// `'static` valid until end of request
// `FnOnce` matches single time request
//
// Use `Box<dyn ...>` to define trait names
type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    // NOTE: implement `Result` error handling via `pub fn build() -> Result<ThreadPool, PoolCreationError>`
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Self { workers, sender }
    }

    // Similar `std::thread::spawn` arguments as we will use it create the thread pool
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        // No fail case as threads continue to execute if pools exists
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || {
            // Looping to keep checking on receiver, `recv()` blocks and waits for input
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {id} executing job!");
                job();
            }
        });
        Self { id, thread } // Since receiver is taken ownership by thread, we can't attribute it
    }
}
