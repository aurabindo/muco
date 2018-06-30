//! Library Management Module

use std::path::Path;
use std::collections::HashMap;
use std::fs;
use walkdir::WalkDir;

use serde_yaml;
use errors::{MucoError, Result};

use super::AudFmt;

static LIB_FILE: &'static str = "library.conf";

#[derive(Serialize, Deserialize, Clone)]
pub struct Library {
    pub count: u32,
    pub exclude: Vec<String>,
    pub index: HashMap<String, Vec<AudFmt>>,
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

fn visit_dirs(ex: Vec<String>) -> Result<Library>{
    let mut file_db: HashMap<String, Vec<AudFmt>> = HashMap::new();
    let mut count: u32 = 0;

    for entry in WalkDir::new(".").into_iter()
        .filter(|ref s| s.is_ok()) {// Get rid of any items with permission errors
            let direntry = entry?;
            let file_name = String::from(direntry.file_name().to_str().unwrap_or(""));
            let path = String::from(direntry.path().to_str().unwrap_or(""));

            if file_name.as_str().starts_with(".") // Ignore hidden items
                || ex.iter().any(|exs| path.as_str().split('/').collect::<Vec<_>>().contains(&exs.as_str())) // Ignore files from excluded path
                || path.as_str().split("./").any(|item| item.starts_with('.')) //ignore if any part of the path contains hidden files
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
                        count += 1;
                    }
                }
        }
    Ok(Library {
        count: count,
        exclude: ex,
        index: file_db,
    })
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

    pub fn uninit() -> Result<()>{
        if Path::new("./.muco").exists() {
            fs::remove_dir_all("./.muco")?
        }

        Ok(())
    }

    pub fn update() -> Result<()> {
        let cfg_file = format!("./.muco/{}", LIB_FILE);

        if !Path::new(cfg_file.as_str()).exists() {
            println!("Sit before you stretch your leg ;)");
            println!("Didnt get it? Initialize the library first!");
        } else {
            let yaml_string = fs::read_to_string(Path::new(&cfg_file));
            let file_lib: Library = serde_yaml::from_str(yaml_string?.as_str())?;
            let disk_lib = visit_dirs(file_lib.exclude.clone())?;
            let disk_db = disk_lib.index.clone();
            let file_db = file_lib.index.clone();


            let missing_path: Vec<_> = disk_db.keys()
                .filter(|f| !file_db.contains_key(f.as_str()))
                .collect();

            println!("Missing paths: {:?}", missing_path);

            if missing_path.len() > 0 {
                let new_config: Library = Library {
                    count: disk_db.len() as u32,
                    exclude: file_lib.exclude,
                    index: disk_db.clone(),
                };

                fs::write(cfg_file, serde_yaml::to_string(&new_config)?)?;
            } else {
                // Inefficient hack: If it gets here, then either
                // nothing has changed, or some previously indexed
                // are missing. So reinit.
                let excl = file_lib.exclude.clone();
                Library::uninit()?;
                Library::init(Some(excl))?;
            }
        }
        Ok(())
    }

    pub fn get() -> Result<Library> {
        let cfg_file = format!("./.muco/{}", LIB_FILE);

        if !Path::new(cfg_file.as_str()).exists() {
            Err(MucoError::Library)
        } else {
            let yaml_string = fs::read_to_string(Path::new(&cfg_file));
            let res = serde_yaml::from_str::<Library>(yaml_string?.as_str())?;
            Ok(res)
        }
    }
}
