use std::fmt;
use std::fmt::Display;

use failure::{Backtrace, Context, Fail};

#[derive(Debug)]
pub struct MucoError {
    inner: Context<MucoErrorKind>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum MucoErrorKind {
    #[fail(display = "Unknown & unfortunate")]
    Unknown,
    #[fail(display = "Resource does not exist")]
    Nonexistant,
    #[fail(display = "Duplicate library")]
    DuplicateLibrary,
    #[fail(display = "Duplicate device")]
    DuplicateDevice,
    #[fail(display = "Cannot read/write configuration file")]
    ConfigurationFile,
    #[fail(display = "Malformed content, cannot serialize/deserialize")]
    Serde,
    #[fail(display = "Transcoding error")]
    Transcode,
}

impl MucoError {
    pub fn kind(&self) -> MucoErrorKind {
        *self.inner.get_context()
    }
}

impl Fail for MucoError {
    fn cause(&self) -> Option<&dyn Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for MucoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl From<MucoErrorKind> for MucoError {
    fn from(kind: MucoErrorKind) -> MucoError {
        MucoError {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<MucoErrorKind>> for MucoError {
    fn from(inner: Context<MucoErrorKind>) -> MucoError {
        MucoError { inner }
    }
}

pub type MucoResult<T> = Result<T, MucoError>;

#[macro_export]
macro_rules! herr {
    ($res:expr) => {
        match $res {
            Ok(_) => (),
            Err(ref err) => {
                eprintln!("{}", my_pretty_failure::myprettyfailure(err));
            }
        }
    };
}

#[macro_export]
macro_rules! herr_exit {
    ($res:expr, $code:expr) => {
        if $res.is_err() {
            herr!($res);
            std::process::exit($code);
        };
    };
}
