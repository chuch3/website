use std::thread;

pub struct ThreadPool {
    size: usize,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        Self { size }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce(),
    {
        todo!();
    }
}
