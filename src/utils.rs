use std::fs::File;
use std::io::Write;
/// Utility functions for use in other modules
use std::path::PathBuf;

use crate::config::Config;
use crate::error::{MucoErrorKind as Kind, MucoResult as Result};
use crate::media::Container;

use failure::ResultExt;
use log::debug;
use walkdir::WalkDir;

// Retrives list of acceptable files from disk
pub(crate) fn get_files<'a>(
    name: &'a str,
    path: &'a PathBuf,
) -> impl Iterator<Item = (PathBuf, Container, &'a PathBuf)> + 'a {
    let base_path = path;
    WalkDir::new(path)
        .into_iter()
        // Skip hidden directories/files in advance
        .filter_entry(|e: &walkdir::DirEntry| {
            !e.file_name()
                .to_str()
                .map(|s| s.starts_with('.'))
                .unwrap_or(false)
        })
        .filter_map(move |e| {
            e.ok().and_then(|dentry| {
                use std::convert::TryFrom;
                let path = dentry.into_path();
                let extension = path
                    .extension()
                    .and_then(|extn| extn.to_str())
                    .unwrap_or_default();
                let container = Container::try_from(extension);
                let is_dir = |p: &PathBuf| p.is_dir();

                if is_dir(&path) {
                    None
                } else {
                    match container {
                        Ok(container) => Some((path, container, base_path)),
                        Err(_err) => {
                            debug!(
                                "Library \"{}\": Unsupported format, skipping {}",
                                &name,
                                path.display()
                            );
                            None
                        }
                    }
                }
            })
        })
}

pub(crate) fn get_config_file() -> Result<PathBuf> {
    let xdg_dirs =
        xdg::BaseDirectories::with_prefix("muco").expect("Cannot create configuration directory");

    match xdg::BaseDirectories::with_prefix("muco")
        .expect("Cannot create configuration directory")
        .find_config_file("muco.toml")
    {
        Some(conf) => Ok(conf),
        None => {
            let path = xdg_dirs
                .place_config_file("muco.toml")
                .expect("Cannot write configuration");

            let mut conf_file = File::create(&path).context(Kind::ConfigurationFile)?;
            let content = toml::to_string(&Config::default()).context(Kind::Serde)?;
            write!(&mut conf_file, "{}", content).context(Kind::ConfigurationFile)?;
            Ok(path)
        }
    }
}
