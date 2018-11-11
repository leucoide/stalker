pub mod events;
pub mod snapshot;

use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

struct WatcherRunner {}

impl WatcherRunner {
    fn watch(&self, watcher: &Watcher) -> events::FSEventEmitter {
        let event_emitter = events::FSEventEmitter::new();
        let watcher_clone = watcher.clone();
        let sender = event_emitter.sender.clone();
        thread::spawn(move || {
            watcher_clone.scan(sender);
        });
        event_emitter
    }
}

#[derive(Clone)]
pub struct Watcher {
    path: PathBuf,
    poll_interval: Duration,
    growth_timeout: Duration,
}

impl Watcher {
    pub fn new(path: String, poll_interval: u64, growth_timeout: u64) -> Watcher {
        Watcher {
            path: PathBuf::from(path),
            poll_interval: Duration::from_millis(poll_interval),
            growth_timeout: Duration::from_millis(growth_timeout),
        }
    }

    fn scan(&self, sender: Sender<events::FSEvent>) {
        let timeout = self.growth_timeout.as_secs();
        let mut old_snapshot = snapshot::FolderSnapShot::from(self.path.clone()).unwrap();
        loop {
            thread::sleep(self.poll_interval);

            let new_snapshot = snapshot::FolderSnapShot::from(self.path.clone()).unwrap();
            for new_path in new_snapshot.metadata.keys() {
                if !old_snapshot.metadata.contains_key(new_path) {
                    match sender.send(events::FSEvent::created(new_path.to_path_buf())) {
                        Ok(()) => {}
                        Err(e) => print!("Error {:?}", e),
                    }
                } else if old_snapshot.metadata[new_path].len()
                    != new_snapshot.metadata[new_path].len()
                {
                    match sender.send(events::FSEvent::modified(new_path.to_path_buf())) {
                        Ok(()) => {}
                        Err(e) => print!("Error {:?}", e),
                    }
                } else if new_snapshot.is_stable(new_path.clone(), timeout)
                    && !old_snapshot.is_stable(new_path.clone(), timeout)
                {
                    match sender.send(events::FSEvent::stabilized(new_path.to_path_buf())) {
                        Ok(()) => {}
                        Err(e) => print!("Error {:?}", e),
                    }
                }
            }
            for old_path in old_snapshot.metadata.keys() {
                if !new_snapshot.metadata.contains_key(old_path) {
                    match sender.send(events::FSEvent::deleted(old_path.to_path_buf())) {
                        Ok(()) => {}
                        Err(e) => print!("Error {:?}", e),
                    }
                }
            }
            old_snapshot = new_snapshot;
        }
    }
    pub fn watch(&self) -> events::FSEventEmitter {
        let runner = WatcherRunner {};
        runner.watch(&self)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
