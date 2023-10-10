pub mod naive;
pub mod rayon;
pub mod shared_queue;

use crate::Result;

pub trait ThreadPool {
    fn new(size: usize) -> Result<Self>
    where
        Self: Sized;

    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static;
}
