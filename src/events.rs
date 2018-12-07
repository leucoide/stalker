use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};

pub enum FSEvent {
    Created(PathBuf),
    Modified(PathBuf),
    Deleted(PathBuf),
    Stabilized(PathBuf),
}

pub struct FSEventEmitter {
    pub sender: Sender<FSEvent>,
    receiver: Receiver<FSEvent>,
}

impl FSEventEmitter {
    pub fn new() -> FSEventEmitter {
        let (tx, rx) = channel();
        return FSEventEmitter {
            sender: tx,
            receiver: rx,
        };
    }
    pub fn listen_sync(&self, callback: fn(FSEvent) -> ()) {
        loop {
            let event = self.receiver.recv().unwrap();
            callback(event)
        }
    }
}
