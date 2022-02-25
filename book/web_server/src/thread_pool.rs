//! Thread Pool to do things in parallel
use std::any::Any;
use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type JobHandler = Box<dyn FnOnce() + Send>;

enum Message {
    Job(JobHandler),
    Terminate,
}

struct Worker {
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(receiver: Arc<Mutex<Receiver<Message>>>) -> Self {
        Worker {
            thread: thread::spawn(move || loop {
                let message = receiver
                    .lock()
                    .expect("Cannot lock channel mutex")
                    .recv()
                    .expect("Cannot resolve channel message");

                match message {
                    Message::Terminate => break,
                    Message::Job(job) => {
                        job();
                    }
                }
            }),
        }
    }

    fn join(self) -> Result<(), Box<dyn Any + Send>> {
        self.thread.join()
    }
}

pub struct ThreadPool {
    workers: Vec<Option<Worker>>,
    sender: mpsc::Sender<Message>,
}

/// ThreadPool of fixed size to execute parallel tasks
///
/// # Examples
///
/// ```rust
/// use std::thread;
/// use std::time::Duration;
/// use std::sync::{Arc, Mutex};
/// use web_server::thread_pool::ThreadPool;
///
/// let counter = Arc::new(Mutex::new(Vec::<u32>::new()));
/// let counter1 = Arc::clone(&counter);
/// let counter2 = Arc::clone(&counter);
/// let counter3 = Arc::clone(&counter);
/// let counter4 = Arc::clone(&counter);
///
/// {
///     let tp = ThreadPool::new(1);
///     tp.execute(move || {
///         thread::sleep(Duration::from_millis(20));
///         counter1.lock().unwrap().push(0);
///     });
///     tp.execute(move || {
///         thread::sleep(Duration::from_millis(10));
///         counter2.lock().unwrap().push(1);
///     });
///     tp.execute(move || {
///         thread::sleep(Duration::from_millis(15));
///         counter3.lock().unwrap().push(2);
///     });
///     tp.execute(move || {
///         counter4.lock().unwrap().push(3);
///     });
/// }

/// assert_eq!(counter.lock().unwrap().as_ref(), vec![0, 1, 2, 3]);
impl ThreadPool {
    /// Creates ThreadPool with the given number of threads
    ///
    /// # Panics
    /// - passing 0 is ambiguous
    pub fn new(thread_number: usize) -> Self {
        assert!(thread_number > 0, "Number of threads must be more than 0");

        let mut workers = Vec::with_capacity(thread_number);

        let (sender, receiver) = mpsc::channel::<Message>();
        let receiver = Arc::new(Mutex::new(receiver));

        for _ in 0..thread_number {
            workers.push(Some(Worker::new(Arc::clone(&receiver))));
        }

        ThreadPool { workers, sender }
    }

    /// Execute job on some of the thread
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::thread;
    /// use std::time::Duration;
    /// use std::sync::{Arc, Mutex};
    /// use web_server::thread_pool::ThreadPool;
    ///
    /// let counter = Arc::new(Mutex::new(Vec::<u32>::new()));
    /// let counter1 = Arc::clone(&counter);
    ///
    /// {
    ///     let tp = ThreadPool::new(1);
    ///     tp.execute(move || {
    ///         thread::sleep(Duration::from_millis(20));
    ///         counter1.lock().unwrap().push(0);
    ///     });
    /// }
    /// ```
    ///
    /// # Panics
    ///
    /// ## Message Channel Errors
    ///
    /// any problem with passing data to the underlying thread leads to the panic
    pub fn execute<T>(&self, job: T)
    where
        T: Fn() -> (),
        T: Send + 'static,
    {
        let job = Box::new(job);

        self.sender
            .send(Message::Job(job))
            .expect("Cannot send message to worker");
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _worker in &self.workers {
            self.sender
                .send(Message::Terminate)
                .expect("Cannot send message to worker");
        }

        for worker in &mut self.workers {
            if let Some(worker) = worker.take() {
                worker.join().expect("Cannot join to the thread");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    #[should_panic(expected = "Number of threads must be more than 0")]
    fn should_throw_when_passed_number_of_threads_is_0() {
        ThreadPool::new(0);
    }

    #[test]
    fn should_execute_1_thread_in_parallel() {
        let counter = Arc::new(Mutex::new(Vec::<u32>::new()));
        let counter1 = Arc::clone(&counter);
        let counter2 = Arc::clone(&counter);
        let counter3 = Arc::clone(&counter);
        let counter4 = Arc::clone(&counter);

        {
            let tp = ThreadPool::new(1);
            tp.execute(move || {
                thread::sleep(Duration::from_millis(20));
                counter1.lock().unwrap().push(0);
            });

            tp.execute(move || {
                thread::sleep(Duration::from_millis(10));
                counter2.lock().unwrap().push(1);
            });

            tp.execute(move || {
                thread::sleep(Duration::from_millis(15));
                counter3.lock().unwrap().push(2);
            });

            tp.execute(move || {
                counter4.lock().unwrap().push(3);
            });
        }

        assert_eq!(counter.lock().unwrap().as_ref(), vec![0, 1, 2, 3]);
    }

    #[test]
    fn should_execute_2_threads_in_parallel() {
        let counter = Arc::new(Mutex::new(Vec::<u32>::new()));
        let counter1 = Arc::clone(&counter);
        let counter2 = Arc::clone(&counter);
        let counter3 = Arc::clone(&counter);
        let counter4 = Arc::clone(&counter);

        {
            let tp = ThreadPool::new(2);
            tp.execute(move || {
                thread::sleep(Duration::from_millis(20));
                counter1.lock().unwrap().push(0);
            });

            tp.execute(move || {
                thread::sleep(Duration::from_millis(10));
                counter2.lock().unwrap().push(1);
            });

            tp.execute(move || {
                thread::sleep(Duration::from_millis(15));
                counter3.lock().unwrap().push(2);
            });

            tp.execute(move || {
                counter4.lock().unwrap().push(3);
            });
        }

        assert_eq!(counter.lock().unwrap().as_ref(), vec![1, 0, 3, 2]);
    }

    #[test]
    fn should_execute_3_threads_in_parallel() {
        let counter = Arc::new(Mutex::new(Vec::<u32>::new()));
        let counter1 = Arc::clone(&counter);
        let counter2 = Arc::clone(&counter);
        let counter3 = Arc::clone(&counter);
        let counter4 = Arc::clone(&counter);

        {
            let tp = ThreadPool::new(3);
            tp.execute(move || {
                thread::sleep(Duration::from_millis(20));
                counter1.lock().unwrap().push(0);
            });

            tp.execute(move || {
                thread::sleep(Duration::from_millis(10));
                counter2.lock().unwrap().push(1);
            });

            tp.execute(move || {
                thread::sleep(Duration::from_millis(15));
                counter3.lock().unwrap().push(2);
            });

            tp.execute(move || {
                counter4.lock().unwrap().push(3);
            });
        }

        assert_eq!(counter.lock().unwrap().as_ref(), vec![1, 3, 2, 0]);
    }

    #[test]
    fn should_execute_4_threads_in_parallel() {
        let counter = Arc::new(Mutex::new(Vec::<u32>::new()));
        let counter1 = Arc::clone(&counter);
        let counter2 = Arc::clone(&counter);
        let counter3 = Arc::clone(&counter);
        let counter4 = Arc::clone(&counter);

        {
            let tp = ThreadPool::new(4);
            tp.execute(move || {
                thread::sleep(Duration::from_millis(20));
                counter1.lock().unwrap().push(0);
            });

            tp.execute(move || {
                thread::sleep(Duration::from_millis(10));
                counter2.lock().unwrap().push(1);
            });

            tp.execute(move || {
                thread::sleep(Duration::from_millis(15));
                counter3.lock().unwrap().push(2);
            });

            tp.execute(move || {
                counter4.lock().unwrap().push(3);
            });
        }

        assert_eq!(counter.lock().unwrap().as_ref(), vec![3, 1, 2, 0]);
    }
}
