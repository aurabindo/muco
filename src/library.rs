///Library handling
use std::collections::HashMap;
use std::path::PathBuf;

use failure::ResultExt;

use crate::config::Config;
use crate::error::{MucoErrorKind as Kind, MucoResult as Result};
use crate::media::Container;
use crate::utils::*;

#[derive(Debug)]
pub struct Library {
    name: String,
    location: PathBuf,
    sources: HashMap<PathBuf, Container>,
}

pub fn add(mut conf: Config, name: String, location: PathBuf) -> Result<()> {
    conf.add_library(name, location)
        .context(Kind::ConfigurationFile)?;
    Ok(())
}

impl Library {
    // fn new(conf: &Config, n: Option<String>) -> Self {
    //     // Get files available on disk
    //     let files_on_disk: HashMap<_, _> = conf
    //         .get_libraries()
    //         .iter()
    //         // Skip traversal for unrequested libraries
    //         .filter_map(|(name, path)| match n {
    //             Some(ref n) => {
    //                 if name.eq(n) {
    //                     Some((name, path))
    //                 } else {
    //                     None
    //                 }
    //             }
    //             None => Some((name, path)),
    //         })
    //         .map(|(name, path)| get_files(name, path).map(|(file, path, base)| {
    //             (file, path)
    //         }))
    //         .flatten()
    //         .collect();

    //     Library {
    //         sources: files_on_disk,
    //     }
    // }

    pub fn location(&self) -> &PathBuf {
        &self.location
    }

    pub fn get(conf: &Config, name: Option<String>) -> Result<Vec<Library>> {
        match name {
            Some(name) => match conf.get_libraries().iter().find(|(n, _)| n.eq(&name)) {
                Some(d) => {
                    let (n, l) = d;
                    let sources: Vec<_> = get_files(n, l).collect();
                    let sources: HashMap<_, _> = sources
                        .into_iter()
                        .map(|(file, container, _base)| (file, container))
                        .collect();

                    Ok(vec![Library {
                        name: n.clone(),
                        location: l.clone(),
                        sources,
                    }])
                }
                None => Err(Kind::Nonexistant)?,
            },
            None => {
                let libraries: Vec<Library> = conf
                    .get_libraries()
                    .iter()
                    .map(|(n, l)| {
                        let sources: Vec<_> = get_files(n, l).collect();
                        let sources: HashMap<_, _> = sources
                            .into_iter()
                            .map(|(file, container, _base)| (file, container))
                            .collect();

                        Library {
                            name: n.clone(),
                            location: l.clone(),
                            sources,
                        }
                    })
                    .collect();

                Ok(libraries)
            }
        }
    }

    pub fn source(self) -> HashMap<PathBuf, Container> {
        self.sources
    }
}
