use std::thread;
use std::sync::{mpsc, Arc, Mutex};
use std::process::exit;
use std::time::Duration;

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

impl ThreadPool {
    //Constructor
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }

    pub fn spawn<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}
#[warn(dead_code)]
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv();
                match  job {
                    Ok(_) => {
                        let job = job.unwrap();
                        job.call_box();
                        thread::sleep(Duration::from_millis(500));
                    }
                    Err(_) => {
                        println!("Terminating.");
                        break;
                    }
                }
            }
        });
        Worker { id, thread }
    }
}

fn main() {
    let pool = ThreadPool::new(4);
    for i in 0..4 {
        pool.spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(250 * i));
            println!("This is Task {}", i);
        });
    }
    std::thread::sleep(std::time::Duration::from_secs(2));
}
