//! Worker pool based on the implementaion from the Rust book.

use crate::{Site, finder::Finder};
use std::{
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver, Sender},
    },
    thread::{self, JoinHandle},
};

pub struct Pool {
    sender: Sender<String>,
    workers: Vec<Worker>,
}

impl Pool {
    pub fn new(size: usize, results: Sender<Site>) -> Self {
        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        let mut workers = Vec::with_capacity(size);

        for _ in 0..size {
            let worker = Worker::new(Arc::clone(&rx), results.clone());
            workers.push(worker);
        }

        Self {
            sender: tx,
            workers,
        }
    }

    pub fn send(&self, url: String) {
        if let Err(e) = self.sender.send(url) {
            log::warn!("sending data failed: {e}");
        }
    }

    pub fn close(self) {
        drop(self.sender);

        for worker in self.workers {
            if let Err(e) = worker.handle.join() {
                log::warn!("worker thread could not be joined {:?}", e);
            }
        }
    }
}

pub struct Worker {
    handle: JoinHandle<()>,
}

impl Worker {
    pub fn new(receiver: Arc<Mutex<Receiver<String>>>, results: Sender<Site>) -> Self {
        let handle = thread::spawn(move || {
            loop {
                let msg = receiver.lock().unwrap().recv();

                match msg {
                    Ok(url) => {
                        let result = Finder::new().find(url);

                        if let Err(_) = results.send(result) {
                            log::warn!("result channel hang up");
                            break;
                        }
                    }
                    Err(_) => {
                        log::warn!("receiver channel hang up");
                        break;
                    }
                }
            }
        });

        Self { handle }
    }
}
