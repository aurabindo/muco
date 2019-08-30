/// Container and media handling
use std::convert::TryFrom;
use std::convert::Into;
use std::fmt;

use crate::error::{MucoError, MucoErrorKind as Kind, MucoResult as Result};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Container {
    Webm,
    Flac,
    Mp3,
}

impl fmt::Display for Container {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "{:?}", self)
    }
}

impl TryFrom<&str> for Container {
    type Error = MucoError;

    fn try_from(val: &str) -> Result<Self> {
        match val.to_ascii_lowercase().as_ref() {
            "flac" => Ok(Container::Flac),
            "mp3" => Ok(Container::Mp3),
            "webm" => Ok(Container::Webm),
            _ => Err(Kind::Unknown)?,
        }
    }
}

impl Into<&'static str> for Container {
    fn into(self) -> &'static str {
        match self {
            Container::Webm => "webm",
            Container::Flac => "flac",
            Container::Mp3 => "mp3",
        }
    }
}

pub trait Transcode {
    /// transcode() must copy the transcoded file to the cache
    /// and update the source Library
    fn transcode(&self, source: Container, target: Container) -> Result<()>;
}
