use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    works: Vec<Work>,
    send: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// size 要大于 0
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (send, rece) = mpsc::channel();

        let rece = Arc::new(Mutex::new(rece));

        // 构造一个至少具有指定容量的新的空 Vec<T>
        let mut works = Vec::with_capacity(size);

        for id in 0..size {
            works.push(Work::new(id, Arc::clone(&rece)));
        }

        ThreadPool { works, send }
    }

    pub fn execute<F>(&self, f: F)
    where
        // thread::spawn 的类型接收这些 所以自定义的也是这些
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.send.send(job).unwrap();
    }
}

pub struct Work {
    id: usize, // usize 指针大小的无符号整数类型 32 位 4 字节 64 位 8 字
    thread: thread::JoinHandle<()>,
}

type Receive = Arc<Mutex<Receiver<Job>>>;

impl Work {
    fn new(id: usize, rece: Receive) -> Work {
        let thread = thread::spawn(move || {
            while let Ok(job) = rece.lock().unwrap().recv() {
                job();
            }
        });

        Work { id, thread }
    }
}
