use anyhow::Result;

use std::fs;
use std::time::SystemTime;
pub struct Script {
    path_to_watch: String,
    last_modified: SystemTime,
}

impl Script {
    pub fn new(path_to_watch: &str) -> Self {
        let last_modified =
            Self::read_metadata(path_to_watch).expect("First metadata reading failed!");
        Self {
            path_to_watch: path_to_watch.to_string(),
            last_modified,
        }
    }

    pub fn update(&mut self) -> bool {
        if let Ok(modified) = Self::read_metadata(&self.path_to_watch) {
            if modified != self.last_modified {
                self.last_modified = modified;
                return true;
            }
        }
        false
    }

    pub fn read_metadata(path: &str) -> Result<SystemTime> {
        let metadata = fs::metadata(path)?;
        assert!(metadata.is_file());
        // TODO return err instead of panicing.
        Ok(metadata.modified()?)
    }
}
