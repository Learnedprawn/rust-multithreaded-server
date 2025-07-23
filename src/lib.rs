use std::{
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver},
    },
    thread::{self},
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
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
        ThreadPool { workers, sender }
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
        self.sender.send(job).unwrap();
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
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("receiver {id} got a job!");
                job();
            }
        });
        Worker { id, handle: thread }
    }
}

pub enum PoolCreationError {
    Error,
}
