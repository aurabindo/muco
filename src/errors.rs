//! Errors Module

use serde_yaml;
use walkdir;
use std::env;

#[derive(Fail, Debug)]
pub enum MucoError {
    #[fail(display = "I/O Error")]
    Io(#[cause] ::std::io::Error),

    #[fail(display = "UTF8 Error")]
    Utf8(#[cause] ::std::str::Utf8Error),

    #[fail(display = "Serde Yaml Error")]
    Yaml(#[cause] ::serde_yaml::Error),

    #[fail(display = "Corrupt Library Error")]
    Library,

    #[fail(display = "Duplicate device being manipulated")]
    DuplicateDevcie,

    #[fail(display = "System Environment Error")]
    SystemEnv,

    #[fail(display = "Audio Transcoding error")]
    Transcode,
}

impl From<serde_yaml::Error> for MucoError {
    fn from(e: serde_yaml::Error) -> Self {
        MucoError::Yaml(e)
    }
}

impl From<::std::io::Error> for MucoError {
    fn from(e: ::std::io::Error) -> Self {
        MucoError::Io(e)
    }
}

impl From<walkdir::Error> for MucoError {
    fn from(_e: walkdir::Error) -> Self {
        MucoError::Library
    }
}

impl From<env::VarError> for MucoError {
    fn from(_e: env::VarError) -> Self {
        MucoError::SystemEnv
    }
}

pub type Result<T> = ::std::result::Result<T, MucoError>;
