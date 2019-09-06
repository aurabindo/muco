use failure::ResultExt;
use std::path::PathBuf;

use log::{error, info};
use serde::{Deserialize, Serialize};
use toml;

use crate::error::MucoErrorKind as Kind;
use crate::error::MucoResult as Result;
use crate::media::Container;
use crate::utils::*;

pub type LibraryConf = Vec<(String, PathBuf)>;
pub type DeviceConf = Vec<(String, PathBuf, Option<String>, Container)>;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Config {
    libraries: LibraryConf,
    devices: DeviceConf,
}

impl Config {
    pub fn add_device(&mut self, n: String, l: PathBuf, e: Option<String>, f: Container) -> Result<()> {
        if self.devices.iter().any(|(_nam, loc, _excl, _f)| loc.eq(&l)) {
            error!("Device already configured at {}", l.display());
            Err(Kind::DuplicateDevice)?
        } else if l.exists() {
            print!("Adding device {} at {}...", &n, &l.display());
            self.devices.push((n, l, e, f));
            self.save().context(Kind::ConfigurationFile)?;
            println!(" Done");
            Ok(())
        } else {
            error!("Bad path {}", l.display());
            Err(Kind::Nonexistant)?
        }
    }

    pub fn add_library(&mut self, name: String, location: PathBuf) -> Result<()> {
        if self.libraries.iter().any(|(_nam, loc)| loc.eq(&location)) {
            error!("Library already configured at {}", location.display());
            Err(Kind::DuplicateLibrary)?
        } else if location.exists() {
            print!("Adding library {} at {}...", &name, &location.display());
            self.libraries.push((name, location));
            self.save().context(Kind::ConfigurationFile)?;
            println!(" Done");
            Ok(())
        } else {
            error!("Bath path {}", location.display());
            Err(Kind::Nonexistant)?
        }
    }

    pub fn get() -> Result<Config> {
        parse_conf(get_config_file().context(Kind::ConfigurationFile)?)
    }

    pub fn get_online(self) -> Result<Config> {
        // Return those devices/libraries which are available(mounted) on the filesystem
        Ok(Config {
            libraries: self
                .libraries
                .into_iter()
                .filter(|(_, path)| path.exists())
                .collect(),
            devices: self
                .devices
                .into_iter()
                .filter(|(_, path, _, _)| path.exists())
                .collect(),
        })
    }

    pub fn get_libraries(&self) -> &LibraryConf {
        &self.libraries
    }

    pub fn get_devices(&self) -> &DeviceConf {
        &self.devices
    }

    fn save(&self) -> Result<()> {
        let config = toml::to_string(&self).context(Kind::Serde)?;
        std::fs::write(get_config_file().context(Kind::ConfigurationFile)?, config)
            .context(Kind::ConfigurationFile)?;
        Ok(())
    }
}

fn parse_conf(c: PathBuf) -> Result<Config> {
    let conf_string = std::fs::read_to_string(&c).context(Kind::ConfigurationFile)?;
    let conf: Config = toml::from_str(&conf_string).context(Kind::Serde)?;

    info!("Configuration successfully read from {}", c.display());
    Ok(conf)
}
