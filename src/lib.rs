//! Muco - Music Copier
//!
//! Muco is a tool useful for synchronising music files across
//! multiple mass storage devices. For example, one may maintain
//! a central music library on the computer. This could be sync'd
//! with few flash drives (one for portable speaker, another for
//! use with car audio), another mobile phone, etc.
//!
//! Although most people would use online streaming services like
//! Spotify or such, there exists people who would want to maintain
//! local copy of the music, likely in a higher quality that those
//! available through the streaming services.
//!
//! Muco can manage multiple libraries, devices, formats, and can
//! transcode on the fly.

use std::process::Command;

use failure::ResultExt;
use log::{debug, error};
use rayon::prelude::*;

pub(crate) mod utils;

pub mod config;
pub mod device;
pub mod error;
pub mod library;
pub mod media;

use config::Config;
use device::Device;
use error::{MucoErrorKind as Kind, MucoResult as Result};
use library::Library;
use media::Container;

pub fn sync(_conf: &Config, dev: Vec<Device>, lib: Vec<Library>) -> Result<()> {
    for library in lib {
        let lib_base = library.location().clone();

        library.source().into_par_iter().for_each(|(path, format)| {
            let stripped_lib_file = path.strip_prefix(&lib_base).ok();

            for dev in dev.iter() {
                let dev_base = dev.location().clone();
                let dev_format = dev.format();
                let dev_format_str: &'static str = dev.format().into();
                let dev_parent = dev_base.join(stripped_lib_file.and_then(|f| f.parent()).unwrap());
                let dev_dest =
                    dev_base.join(stripped_lib_file.unwrap().with_extension(dev_format_str));

                let found = dev.source().iter().any(|(dev_file, _)| {
                    let filename = dev_file.strip_prefix(&dev_base).unwrap();
                    stripped_lib_file.unwrap().eq(filename)
                });

                if found {
                    continue;
                } else if dev_format.eq(&format) {
                    println!("Copy to {}", dev_dest.display());
                    debug!("Copying: {} to {}", path.display(), dev_dest.display());
                    herr_exit!(std::fs::copy(&path, &dev_dest).context(Kind::Unknown), 1);
                } else {
                    println!("Transcoding to: {}", dev_dest.display());
                    debug!("Creating directories: {}", dev_parent.display());
                    herr_exit!(std::fs::create_dir_all(&dev_parent), 1);

                    match dev_format {
                        Container::Mp3 => {
                            let output = Command::new("ffmpeg")
                                .current_dir(".")
                                .arg("-i")
                                .arg(&path)
                                .arg("-vn")
                                .arg("-codec:a")
                                .arg("libmp3lame")
                                .arg("-b:a")
                                .arg("320k")
                                .arg(&dev_dest)
                                .output();

                            herr_exit!(output, 1);
                        }
                        Container::Flac => {
                            let output = Command::new("ffmpeg")
                                .current_dir(".")
                                .arg("-i")
                                .arg(&path)
                                .arg("-acodec")
                                .arg("flac")
                                .arg("-bits_per_raw_sample")
                                .arg("24")
                                .arg("-ar")
                                .arg("48000")
                                .arg(&dev_dest)
                                .output();

                            herr_exit!(output, 1);
                        }
                        _ => {
                            error!("Transcoding not supported to {:?}", dev_format);
                            herr_exit!(Err(Kind::Transcode.into()) as Result<()>, 1);
                        }
                    }
                }
            }
        })
    }
    Ok(())
}
