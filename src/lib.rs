extern crate failure;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_yaml;

#[derive(Serialize, Deserialize)]
pub enum AudFmt {
    Aac,
    Flac,
    Mp3,
    Wma,
}

mod errors;
mod library;
mod device;
