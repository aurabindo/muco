//! Library Management Module

use std::path::{PathBuf, Path};
use std::collections::HashMap;
use std::fs;

use serde_yaml;
use failure;
use errors::{MucoError, Result};

use super::AudFmt;

static LIB_FILE: &'static str = "library.conf";

#[derive(Serialize, Deserialize)]
pub struct Library {
    pub count: u32,
    pub size: usize,
    index: HashMap<String, Vec<AudFmt>>,
}

// In case there is an existing library
fn validate_library(lib: &Library) -> Result<()>{
    for (path, _) in lib.index.iter() {
        if !Path::new(path).exists() {
            return Err(MucoError::Library)
        }
    }
    Ok(())
}

impl Library {
    // If library exists, validate
    // Else, initialize a new one
    pub fn init() -> Result<Library> {
        let cfg_file = format!("./.muco/{}", LIB_FILE);

        if Path::new(cfg_file.as_str()).exists() {
            let lib_snapshot = fs::read_to_string(cfg_file.as_str())?;
            let lib: Library = serde_yaml::from_str(lib_snapshot.as_str())?;
            validate_library(&lib)?;
            return Ok(lib)
        }

        if !Path::new("./.muco").exists() {
            fs::create_dir("./.muco")?;
        }

        let lib_snapshot = Library {
            count: 0,
            size: 0,
            index: HashMap::new(),
        };

        let lib_snapshot_ser = serde_yaml::to_string(&lib_snapshot)?;

        fs::write(cfg_file, lib_snapshot_ser)?;
        Ok(lib_snapshot)
    }

    pub fn uninit(self) -> Result<()>{
        if Path::new("./.muco").exists() {
            fs::remove_dir_all("./.muco")?
        }

        Ok(())
    }

    pub fn update(self) {
    }
}
