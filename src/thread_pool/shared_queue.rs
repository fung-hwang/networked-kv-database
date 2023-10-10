use crate::{Result, ThreadPool};
use crossbeam_channel;
use std::thread;

// If a thread in thread pool panics, let the thread die and spawn another(thread::panicking),
// or catch the panic and keep the existing thread running(catch_unwind):
// Note for Rust training course: the thread pool is not implemented using
// `catch_unwind` because it would require the Job to be `UnwindSafe`.

type Job = Box<dyn FnOnce() + Send + 'static>;

enum ThreadPoolMessage {
    NewJob(Job),
    Terminate,
}

/// A thread pool using a shared queue inside.
pub struct SharedQueueThreadPool {
    workers: Vec<Worker>,
    sender: crossbeam_channel::Sender<ThreadPoolMessage>,
}

impl ThreadPool for SharedQueueThreadPool {
    fn new(size: usize) -> Result<Self>
    where
        Self: Sized,
    {
        assert!(size > 0);
        let (sender, receiver) = crossbeam_channel::unbounded();
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(
                id,
                ThreadPoolMessageReceiver {
                    receiver: receiver.clone(),
                },
            ));
        }

        Ok(Self { workers, sender })
    }

    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(job);
        self.sender
            .send(ThreadPoolMessage::NewJob(job))
            .expect("Unable to send threadpool message(job)...");
    }
}

impl Drop for SharedQueueThreadPool {
    fn drop(&mut self) {
        for _ in &mut self.workers {
            self.sender
                .send(ThreadPoolMessage::Terminate)
                .expect("Unable to send threadpool message(terminal)...");
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                if !thread.is_finished() {
                    thread.join().expect("Unable to join thread...");
                }
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, msg_receiver: ThreadPoolMessageReceiver) -> Worker {
        let thread = thread::spawn(move || msg_receiver.receive_and_execute_job());

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

#[derive(Clone)]
struct ThreadPoolMessageReceiver {
    receiver: crossbeam_channel::Receiver<ThreadPoolMessage>,
}

impl ThreadPoolMessageReceiver {
    fn receive_and_execute_job(self) {
        loop {
            let message = self.receiver.recv().expect("Unable to receive message...");

            match message {
                ThreadPoolMessage::NewJob(job) => job(),
                ThreadPoolMessage::Terminate => {
                    break;
                }
            }
        }
    }
}

impl Drop for ThreadPoolMessageReceiver {
    fn drop(&mut self) {
        // If a thread in thread pool panics, let the thread die and spawn another.
        if thread::panicking() {
            // FIXME: Thread created here can't receive terminate message.
            // But how to add thread to workers in threadpool?
            let msg_receiver = self.clone();
            thread::spawn(move || msg_receiver.receive_and_execute_job());
        }
    }
}
