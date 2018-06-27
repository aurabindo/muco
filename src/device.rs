//! Device Management

use std::path::PathBuf;
use std::collections::HashMap;

use super::AudFmt;

pub struct Device {
    pub capacity: usize,
    pub audio_formats: Vec<AudFmt>,
    // Index of the copied files
    pub index: HashMap<PathBuf, AudFmt>,
}

impl Device {
    pub fn init() -> Device {
        unimplemented!()
    }

    pub fn uninit(self) {
        unimplemented!()
    }

    pub fn validate_index(self) {
        unimplemented!()
    }

    // Synchronize the Library with this device
    pub fn sync(self) {
        unimplemented!()
    }
}
