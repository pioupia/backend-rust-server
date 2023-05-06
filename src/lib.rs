use std::sync::{Arc, mpsc, Mutex};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, &'static str> {
        if size == 0 {
            return Err("The ThreadPool size should be more than 0.");
        }

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // create some threads and store them in the vector
            let new_worker = Worker::new(id, Arc::clone(&receiver));
            match new_worker {
                Err(_) => (),
                Ok(w) => workers.push(w)
            }
        }

        Ok(ThreadPool { workers, sender })
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Result<Worker, &'static str> {
        let builder = thread::Builder::new();
        let thread = match builder.spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            job();
        }) {
            Ok(t) => t,
            Err(e) => {
                println!("An error has occurred when created a new thread: {}", e);
                return Err("An error has occurred when created a new thread");
            }
        };

        Ok(Worker { id, thread })
    }
}