//! Errors Module

use serde_yaml;

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
pub type Result<T> = ::std::result::Result<T, MucoError>;
