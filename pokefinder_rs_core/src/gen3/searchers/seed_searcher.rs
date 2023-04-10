use std::sync::atomic::{AtomicBool, AtomicU32};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct SeedSearcher<P: Send + Sync> {
    pub progress: Arc<AtomicU32>,
    pub results: Arc<Mutex<Vec<u32>>>,
    pub criteria: Arc<P>,
    pub searching: Arc<AtomicBool>,
}

impl<P: Send + Sync> SeedSearcher<P> {
    pub fn new(criteria: P) -> Self {
        Self {
            progress: Arc::new(AtomicU32::new(0)),
            results: Arc::new(Mutex::new(vec![])),
            criteria: Arc::new(criteria),
            searching: Arc::new(AtomicBool::new(false)),
        }
    }
}
