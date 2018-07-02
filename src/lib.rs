extern crate failure;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_yaml;
extern crate walkdir;

use std::str::FromStr;
use std::convert::AsRef;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum AudFmt {
    Aac,
    Flac,
    Mp3,
    Wma,
}

impl FromStr for AudFmt {
    type Err = ();

    fn from_str(s: &str) -> Result<AudFmt, ()> {
        match s.to_lowercase().as_str() {
            "aac" => Ok(AudFmt::Aac),
            "flac" => Ok(AudFmt::Flac),
            "mp3" => Ok(AudFmt::Mp3),
            "wma" => Ok(AudFmt::Wma),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AudFmt {
    fn as_ref(&self) -> &str {
        match *self {
            AudFmt::Flac => "flac",
            AudFmt::Mp3 => "mp3",
            AudFmt::Aac => "aac",
            AudFmt::Wma => "wma",
        }
    }
}

pub mod errors;
pub mod library;
pub mod device;
