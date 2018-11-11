use std::collections;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Clone)]
pub struct FolderSnapShot {
    pub path: PathBuf,
    pub metadata: collections::HashMap<PathBuf, fs::Metadata>,
    pub timestamp: SystemTime,
}

impl FolderSnapShot {
    pub fn from(path: PathBuf) -> io::Result<FolderSnapShot> {
        let mut snapshot = FolderSnapShot {
            path: path.clone(),
            metadata: collections::HashMap::new(),
            timestamp: SystemTime::now(),
        };
        for entry in fs::read_dir(&path)? {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    let entry_path = entry.path();
                    snapshot.metadata.insert(entry_path, metadata);
                }
            }
        }

        Ok(snapshot)
    }

    pub fn is_stable(&self, path: PathBuf, timeout: u64) -> bool {
        let metadata = path.metadata().unwrap();
        let last_modify = metadata.modified().unwrap();
        self.timestamp
            .duration_since(last_modify)
            .unwrap()
            .as_secs()
            > timeout
    }
}
