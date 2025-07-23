use std::{
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver},
    },
    thread::{self},
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in self.workers.drain(..) {
            println!("shutting down worker: {}", worker.id);
            worker.handle.join().unwrap();
        }
    }
}
type Job = Box<dyn FnOnce() + Send + 'static>;
impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(number_of_threads: usize) -> ThreadPool {
        assert!(number_of_threads > 0);
        let mut workers = Vec::with_capacity(number_of_threads);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for i in 0..number_of_threads {
            let worker = Worker::new(i, Arc::clone(&receiver));
            workers.push(worker);
        }
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    // pub fn build(number_of_threads: usize) -> Result<ThreadPool, PoolCreationError> {
    //     assert!(number_of_threads > 0);
    //     let mut workers = Vec::with_capacity(number_of_threads);
    //
    //     for _ in 0..number_of_threads {}
    //     Ok(ThreadPool { workers })
    // }

    pub fn execute<C>(&self, closure: C)
    where
        C: FnOnce() + Send + 'static,
    {
        let job = Box::new(closure);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    handle: thread::JoinHandle<()>,
    // receiver: mpsc::Receiver<Job>,
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv();
                match message {
                    Ok(job) => {
                        println!("receiver {id} got a job!");
                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} shut down");
                        break;
                    }
                }
            }
        });
        Worker { id, handle: thread }
    }
}

pub enum PoolCreationError {
    Error,
}
