use crate::Result;
use rayon::{ThreadPool, ThreadPoolBuilder};

/// Rayon threadpool encapsulation
pub struct RayonThreadPool {
    pool: ThreadPool,
}

impl crate::ThreadPool for RayonThreadPool {
    fn new(size: usize) -> Result<Self> {
        let pool = ThreadPoolBuilder::new().num_threads(size).build()?;

        Ok(Self { pool })
    }

    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.pool.spawn(job)
    }
}
