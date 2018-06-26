//! Library Management Module

use std::path::Path;
use std::collections::HashMap;
use std::fs;
use walkdir::WalkDir;

use serde_yaml;
use errors::{MucoError, Result};

use super::AudFmt;

static LIB_FILE: &'static str = "library.conf";

#[derive(Serialize, Deserialize)]
pub struct Library {
    pub count: u32,
    pub size: usize,
    pub exclude: Vec<String>,
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

fn visit_dirs(ex: Vec<String>) -> Result<HashMap<String,Vec<AudFmt>>>{
    let mut file_db: HashMap<String, Vec<AudFmt>> = HashMap::new();

    for entry in WalkDir::new(".").into_iter()
        .filter(|ref s| s.is_ok()) {// Get rid of any items with permission errors
            let direntry = entry?;
            let file_name = String::from(direntry.file_name().to_str().unwrap_or(""));
            let path = String::from(direntry.path().to_str().unwrap_or(""));

            if file_name.as_str().starts_with(".") // Ignore hidden items
                || ex.iter().any(|exs| path.as_str().split('/').collect::<Vec<_>>().contains(&exs.as_str())) // Ignore files from excluded path
                || file_name.as_str().split('.').count() < 2 { //There must be atleast 1 dot in the file name
                    continue;
                } else {
                    if let Ok(fmt) = file_name.as_str().split('.').last().unwrap().parse::<AudFmt>() {
                        // Warning: Terrible hack:
                        // if statement above ensures last().unwrap() does not fail.
                        if file_db.contains_key(&path) {
                            if let Some(x) = file_db.get_mut(&path) {
                                x.push(fmt);
                            }
                        } else {
                            file_db.insert(path, vec![fmt]);
                        }
                    }
                }
        }
    Ok(file_db)
}

impl Library {
    // If library exists, validate
    // Else, initialize a new one
    pub fn init(excludes: Option<Vec<String>>) -> Result<()> {
        let cfg_file = format!("./.muco/{}", LIB_FILE);

        if Path::new(cfg_file.as_str()).exists() {
            let lib_snapshot = fs::read_to_string(cfg_file.as_str())?;
            let lib: Library = serde_yaml::from_str(lib_snapshot.as_str())?;
            validate_library(&lib)?;
            return Ok(())
        }

        if !Path::new("./.muco").exists() {
            fs::create_dir("./.muco")?;

            let res = excludes
                .and_then(|list| visit_dirs(list).ok())
                .and_then(|ref items| serde_yaml::to_string(items).ok());

            if let Some(x) = res {
                fs::write(format!("./.muco/{}", LIB_FILE), x)?;
            }
        }

        Ok(())
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
