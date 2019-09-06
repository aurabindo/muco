/// Device Handling
use std::collections::HashMap;
use std::convert::TryFrom;
use std::path::PathBuf;

use crate::config::Config;
use crate::error::{MucoErrorKind as Kind, MucoResult as Result};
use crate::media::Container;
use crate::utils::*;

#[derive(Debug)]
pub struct Device {
    name: String,
    location: PathBuf,
    excludes: Option<String>,
    format: Container,
    sources: HashMap<PathBuf, Container>,
}

pub fn add(
    mut conf: Config,
    name: String,
    location: PathBuf,
    exclude: Option<String>,
    format: String,
) -> Result<()> {
    let format = Container::try_from(format.as_str())?;
    conf.add_device(name, location, exclude, format)?;
    Ok(())
}

impl Device {
    pub fn location(&self) -> &PathBuf {
        &self.location
    }

    pub fn source(&self) -> &HashMap<PathBuf, Container> {
        &self.sources
    }

    pub fn format(&self) -> Container {
        self.format
    }

    pub fn get(conf: &Config, name: Option<String>) -> Result<Vec<Device>> {
        match name {
            Some(name) => match conf.get_devices().iter().find(|(n, _, _, _)| n.eq(&name)) {
                Some(d) => {
                    let (n, l, e, f) = d;
                    let sources: Vec<_> = get_files(n, l).collect();
                    let sources: HashMap<_, _> = sources
                        .into_iter()
                        .map(|(file, container, _base)| (file, container))
                        .collect();

                    Ok(vec![Device {
                        name: n.clone(),
                        location: l.clone(),
                        excludes: e.clone(),
                        format: *f,
                        sources,
                    }])
                }
                None => Err(Kind::Nonexistant)?,
            },
            None => {
                let devices: Vec<Device> = conf
                    .get_devices()
                    .iter()
                    .map(|(n, l, e, f)| {
                        let sources: Vec<_> = get_files(n, l).collect();
                        let sources: HashMap<_, _> = sources
                            .into_iter()
                            .map(|(file, container, _base)| (file, container))
                            .collect();

                        Device {
                            name: n.clone(),
                            location: l.clone(),
                            excludes: e.clone(),
                            format: *f,
                            sources,
                        }
                    })
                    .collect();

                Ok(devices)
            }
        }
    }
}
