use crate::{model::Model, process::ProcessIterator};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    thread::JoinHandle,
    time::Duration,
};

pub struct Worker {
    thread: Option<JoinHandle<()>>,
    state: Arc<AtomicBool>,
}

impl Worker {
    pub fn new() -> Self {
        Self {
            thread: None,
            state: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn get_thread(&self) -> Option<&JoinHandle<()>> {
        self.thread.as_ref()
    }

    pub fn start(&mut self) {
        // Check if the thread is already running
        if self.thread.is_some() {
            return;
        }

        // Clone the Arc for the thread closure
        let state = self.state.clone();
        self.thread = Some(thread::spawn(move || {
            let mut model = Model::new();
            while !state.load(Ordering::Relaxed) {
                ProcessIterator::new(vec![]).iterate(&mut model);
                thread::sleep(Duration::from_secs(1))
            }
        }));
    }

    pub fn stop(&mut self) {
        // Set the state flag to signal the thread to stop
        self.state.store(true, Ordering::Relaxed);
        // If the thread is running, wait for it to finish
        if let Some(thread) = self.thread.take() {
            thread.join().unwrap(); // You might want to handle errors gracefully
        }
    }
    pub fn is_running(&self) -> bool {
        self.thread.is_some()
    }
}
