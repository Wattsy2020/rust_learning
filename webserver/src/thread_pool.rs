use pooled_thread::PooledThread;
use std::sync::mpsc;
use std::thread;
use thread_id::ThreadId;

// note: functions need to live for the static lifetime
// since they could be waiting for an indefinite amount of time before the thread executes them
type Function = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    threads: Vec<PooledThread>,
    thread_complete_receiver: mpsc::Receiver<ThreadId>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(num_threads: u32) -> ThreadPool {
        assert!(
            num_threads > 0,
            "Must provide more than 0 threads for work to be done"
        );
        let (sender, receiver) = mpsc::channel::<ThreadId>();
        let threads = (0..num_threads)
            .map(|id| PooledThread::start(ThreadId::new(id), sender.clone()))
            .collect();
        ThreadPool {
            threads,
            thread_complete_receiver: receiver,
        }
    }

    pub fn execute<F: FnOnce() + Send + 'static>(&self, work: F) {
        let ready_id = self.thread_complete_receiver.recv().unwrap();
        self.threads[ready_id.id() as usize].execute(Box::new(work));
    }

    // todo: implement drop to terminate all the threads
}

mod pooled_thread {
    use super::*;

    /// A thread owned by the thread pool, with a way to send work to it
    pub struct PooledThread {
        thread: thread::JoinHandle<()>,
        work_sender: mpsc::Sender<Function>,
    }

    impl PooledThread {
        /// Start a pooled thread that polls for work, returning a handle that sends the thread work
        pub fn start(thread_id: ThreadId, work_complete_sender: mpsc::Sender<ThreadId>) -> Self {
            let (sender, receiver) = mpsc::channel::<Function>();
            let thread = thread::spawn(move || {
                work_complete_sender.send(thread_id).unwrap();
                for work in receiver {
                    work();
                    work_complete_sender.send(thread_id).unwrap();
                }
            });
            PooledThread {
                thread,
                work_sender: sender,
            }
        }

        /// Execute work on the thread
        pub fn execute(&self, work: Function) {
            self.work_sender.send(work).unwrap()
        }
    }
}

mod thread_id {
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub struct ThreadId {
        id: u32,
    }

    impl ThreadId {
        pub fn new(id: u32) -> ThreadId {
            ThreadId { id }
        }

        pub fn id(&self) -> u32 {
            self.id
        }
    }
}
