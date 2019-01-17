//! Device Management

use std::path::{PathBuf, Path};
use std::fs;
use std::collections::HashMap;
use std::env;
use std::process::Command;

use serde_yaml;
use errors::Result ;
use errors::MucoError;

use super::AudFmt;
use super::library::Library;

static DEV_DB: &'static str = ".muco.db";
static MUCO_CFG: &'static str = "/.mucorc";

macro_rules! muco_cfg_file {
    () => {
        {
            let mut home = match env::var("HOME") {
                Ok(path) => {
                    path
                },
                Err(_e) => {
                    "./".to_string()
                },
            };

            home.push_str(MUCO_CFG);
            home
        }

    };
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Device {
    pub name: String,
    pub location: PathBuf,
    pub formats: Vec<AudFmt>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub devices: Vec<Device>,
}

impl Config {
    pub fn init() -> Result<Self> {
        // Lets read the config file first
        if !Path::new(&muco_cfg_file!()).exists() {
            return Ok(Config {
                devices: vec![],
            })
        }

        let known_dev: Config = serde_yaml::from_str(&fs::read_to_string(muco_cfg_file!())?)?;

        // Lets check if all the known devices exist currently.
        // No need to write the new config to disk, since those devices
        // that are absent, maybe plugged in later.
        let available_dev: Vec<_> = known_dev.devices.into_iter()
            .filter(|item| item.location.exists()) // discard if not mounted
            .filter(|item| { // discard if not writeable
                match fs::metadata(item.clone().location) {
                    Ok(metadata) => !metadata.permissions().readonly(),
                    Err(_) => false,
                }
            })
            .collect();

        let available = Config {
            devices: available_dev,
        };

        Ok(available)
    }

    pub fn add_device(&mut self, dev: &Device) -> Result<()> {
        if self.devices.iter()
            .filter(|item| item.name.eq(&dev.name))
            .count() > 0 {
                Err(MucoError::DuplicateDevcie)
            } else {
            self.devices.push(dev.clone());
            update_config(&self)?;

            Ok(())
        }
    }

    pub fn list_devices(&self) -> Result<()> {
        println!("Current devices:\n{:?}", self);
        Ok(())
    }

    pub fn remove_device(&mut self, name: &String) -> Result<()>{
        let removed = self.devices.iter()
            .position(|item| item.name.eq(name))
            .map(|item| self.devices.remove(item))
            .is_some();

        if removed {
            update_config(&self)?;
        }

        Ok(())
    }

    pub fn sync(&self, dev: &Device) -> Result<()> {
        println!("Total number of devices to sync: {:?}", self.devices.len());
        let lib = Library::get()?;

        if !Path::new(&format!("{}/{}", dev.location.display(), DEV_DB)).exists() {
            dev_create_index_copy(dev, lib.clone())?;
            return Ok(())
        } else {
            let dev_db_path = dev.location.clone().join(DEV_DB);
            let mut existing_files: HashMap<String, ()> = serde_yaml::from_str(&fs::read_to_string(dev_db_path.clone())?)?;

            verify_dev_files(&mut existing_files, &dev.location);

            for (file, fmt) in lib.index.iter() {
                for f in fmt {
                    if !dev.formats.contains(f) {
                        convert_and_copy(file.clone(), dev.formats.first().unwrap().clone(), dev.location.clone(), dev.name.clone())?;
                        break;
                    }
                }

                if fmt.iter()
                    .filter(|f| dev.formats.contains(f))
                    .count() > 0 {
                        if !existing_files.contains_key(file) {
                            let path_for_parent = dev.location.join(file);
                            let parent = path_for_parent.parent();

                            if let Some(path) = parent {
                                fs::create_dir_all(path)?;
                                fs::copy(file, dev.location.join(file))?;
                                existing_files.insert(file.to_string(), ());
                                println!("{}: Copying: {}", dev.name, file);
                            }
                        }
                    }
            }

            // println!("To convert: {:?}", to_convert);
            fs::write(dev.location.join(DEV_DB), serde_yaml::to_string(&existing_files)?)?;

            Ok(())
        }
    }
}

fn verify_dev_files(idx: &mut HashMap<String, ()>, file: &PathBuf) {
    let to_remove: Vec<_> = idx.iter()
        .filter(|&(f, _)| !file.join(f).exists())
        .map(|(k, _)| k.clone())
        .collect();

    for item in to_remove {
        &mut idx.remove(&item);
    }
}

fn convert_and_copy(file: String, to_fmt: AudFmt, loc: PathBuf, name: String) -> Result<()> {

    let last_dot = file.as_str().rfind('.').unwrap_or(file.len());

    let file_name = &file.as_str()[..last_dot];

    if let Some(parent) = Path::new(file_name).parent() {
        fs::create_dir_all(loc.join(parent))?;
    }

    let mut file_target = String::from(file_name);

    file_target.push_str(".");
    file_target.push_str(to_fmt.as_ref());

    println!("{}: Converting & Copying {}", name, file);

    if !loc.join(file_target.clone()).exists() {
        let output = Command::new("ffmpeg")
            .current_dir(".")
            .arg("-vn")
            .arg("-i")
            .arg(file.as_str())
            .arg(loc.join(file_target))
            .output()
            .expect("Could not transcode");

        if output.status.success() {
            Ok(())
        } else {
            Err(MucoError::Transcode)
        }
    } else {
        Ok(())
    }
}

fn dev_create_index_copy(dev: &Device, l: Library) -> Result<()> {
    let mut dev_file_index: HashMap<String, ()> = HashMap::new();

    for (path, fmt) in l.index.iter() {
        for f in fmt {
            if !dev.formats.contains(f) {
                convert_and_copy(path.clone(), dev.formats.first().unwrap().clone(), dev.location.clone(), dev.name.clone())?;
                break;
            }
        }
        if fmt.iter()
            .filter(|f| dev.formats.contains(f))
            .count() > 0 {
                let to_path = dev.location.join(path.clone());
                let for_parent = to_path.clone();
                let from_path = path.clone();
                let to_path_parent = for_parent.parent();

                if let Some(path) = to_path_parent {
                    fs::create_dir_all(path)?;
                }

                println!("{}: Copying {}", dev.name, from_path);
                fs::copy(path.clone(), to_path)?;

                dev_file_index.insert(from_path, ());
            }
    }

    let dev_db_path = dev.location.join(DEV_DB);
    fs::write(dev_db_path, serde_yaml::to_string(&dev_file_index)?)?;

    Ok(())
}

fn update_config(dev: &Config) -> Result<()> {
    if Path::new(&muco_cfg_file!()).exists() {
        fs::remove_file(muco_cfg_file!())?;
    }

    let to_write = serde_yaml::to_string(dev)?;
    fs::write(muco_cfg_file!(), to_write)?;

    Ok(())
}
