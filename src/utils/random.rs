use rand::distributions::{Distribution, Uniform};
use rand::{rngs::ThreadRng, thread_rng};
use std::cell::RefCell;
use std::sync::{Arc, Mutex};

pub struct Random {
    rng: Mutex<RefCell<ThreadRng>>,
}

impl Random {
    pub fn new() -> Self {
        Self {
            rng: Mutex::new(RefCell::new(thread_rng())),
        }
    }

    pub fn random(&self, start: usize, end: usize) -> usize {
        let _lock = self.rng.lock().unwrap();
        let mut seed = _lock.borrow_mut();
        let between = Uniform::from(start..end);
        between.sample(&mut *seed)
    }

    pub fn random_chance(&self) -> f64 {
        (self.random(0, usize::MAX) as f64) / (usize::MAX as f64)
    }

    pub fn exp_dist(&self, prob: f64, max_value: usize) -> usize {
        let rng = self.random_chance();
        std::cmp::min(rng.log(prob).trunc() as usize, max_value)
    }

    pub fn random_pick<'a, T>(&self, options: &'a [T]) -> &'a T {
        let pos = self.random(0, options.len());
        &options[pos]
    }
}

unsafe impl Send for Random {}
unsafe impl Sync for Random {}
