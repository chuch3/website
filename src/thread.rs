use crate::context::Context;
use std::sync::{Arc, Mutex, mpsc};
use std::thread::{self, JoinHandle};

// `Send` transfer between threads
// `'static` valid until end of request
// `FnOnce` matches single time request
//
// Use `Box<dyn ...>` to define trait names
type Job = Box<dyn FnOnce(&Context) + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let workers = (0..size)
            .map(|i| Worker::new(i, Arc::clone(&receiver)))
            .collect();

        Self {
            workers,
            sender: Some(sender),
        }
    }

    // Similar `std::thread::spawn` arguments as we will use it create the thread pool
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce(&Context) + Send + 'static,
    {
        let job = Box::new(f);
        // No fail case as threads continue to execute if pools exists
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in self.workers.drain(..) {
            println!("Stopping worker {}", worker.id);
            worker.thread.join().unwrap();
        }
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
                let message = receiver
                    .lock()
                    .expect("Error! Mutex in poisoned state from panicked threads!")
                    .recv();
                let thread_ctx = Context::new();

                match message {
                    Ok(job) => {
                        println!("Worker {id} executing job!");
                        job(&thread_ctx);
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });
        Self { id, thread } // Since receiver is taken ownership by thread, we can't attribute it
    }
}
