use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};

#[derive(Debug)]
pub enum FSeventType {
    Created,
    Modified,
    Deleted,
    Stabilized,
}

pub struct FSEvent {
    pub event_type: FSeventType,
    pub path: PathBuf,
}

impl FSEvent {
    pub fn created(path: PathBuf) -> FSEvent {
        FSEvent {
            event_type: FSeventType::Created,
            path: path,
        }
    }
    pub fn modified(path: PathBuf) -> FSEvent {
        FSEvent {
            event_type: FSeventType::Modified,
            path: path,
        }
    }
    pub fn deleted(path: PathBuf) -> FSEvent {
        FSEvent {
            event_type: FSeventType::Deleted,
            path: path,
        }
    }
    pub fn stabilized(path: PathBuf) -> FSEvent {
        FSEvent {
            event_type: FSeventType::Stabilized,
            path: path,
        }
    }
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
