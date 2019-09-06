/// Muco Cli frontend
use std::path::PathBuf;

use muco::error::{MucoErrorKind as Kind, MucoResult as Result};
use muco::herr;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use env_logger;
use failure::ResultExt;

fn main() {
    env_logger::init();

    let matches = App::new("Muco")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Music transcoding and local synchronization tool")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::InferSubcommands)
        .setting(AppSettings::ArgsNegateSubcommands)
        .setting(AppSettings::UnifiedHelpMessage)
        .setting(AppSettings::DisableHelpSubcommand)
        .subcommand(
            SubCommand::with_name("library")
                .about("Manage libraries")
                .subcommand(
                    SubCommand::with_name("add")
                        .arg(
                            Arg::with_name("location")
                                .short("l")
                                .required(false)
                                .takes_value(true),
                        )
                        .arg(
                            Arg::with_name("name")
                                .short("n")
                                .required(true)
                                .takes_value(true),
                        ),
                ),
        )
        .subcommand(
            SubCommand::with_name("sync")
                .about("Synchronize Devices with Libraries")
                .arg(
                    Arg::with_name("library")
                        .short("l")
                        .required(false)
                        .multiple(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("device")
                        .short("d")
                        .required(false)
                        .multiple(false)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("device")
                .about("Manage devices")
                .subcommand(SubCommand::with_name("list").alias("show"))
                .subcommand(
                    SubCommand::with_name("add")
                        .arg(
                            Arg::with_name("location")
                                .short("l")
                                .long("location")
                                .takes_value(true)
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("name")
                                .short("n")
                                .long("name")
                                .takes_value(true)
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("format")
                                .short("f")
                                .long("format")
                                .takes_value(true)
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("exclude")
                                .short("e")
                                .long("exclude")
                                .takes_value(true)
                                .multiple(true)
                                .required(false),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("remove")
                        .arg(
                            Arg::with_name("location")
                                .short("l")
                                .required(true)
                                .conflicts_with("name"),
                        )
                        .arg(
                            Arg::with_name("name")
                                .short("n")
                                .required(true)
                                .conflicts_with("location"),
                        ),
                ),
        )
        .subcommand(SubCommand::with_name("sync").about("Synchronize managed devices & libraries"))
        .get_matches();

    match matches.subcommand() {
        ("device", Some(device)) => {
            herr!(handle_device(device));
        }
        ("library", Some(library)) => {
            herr!(handle_library(library));
        }
        ("sync", Some(sync)) => {
            herr!(handle_sync(sync));
        }
        _ => (),
    }
}

fn handle_device(dev: &ArgMatches) -> Result<()> {
    match dev.subcommand() {
        ("add", Some(m)) => {
            // Can unwrap here safely because of clap configuration
            let fmt = m.value_of("format").unwrap().to_owned();
            let nam = m.value_of("name").unwrap().to_owned();

            let loc = PathBuf::from(m.value_of("location").unwrap_or(env!("PWD")));
            let excl = m.value_of("exclude").map(|excl| excl.to_owned());

            let conf = muco::config::Config::get()?;
            muco::device::add(conf, nam, loc, excl, fmt)?;

            Ok(())
        }
        //TODO: Add device list/status
        _ => unimplemented!(),
    }
}

fn handle_library(lib: &ArgMatches) -> Result<()> {
    match lib.subcommand() {
        ("add", Some(m)) => {
            let loc = m.value_of("location").unwrap_or(env!("PWD"));
            // Can unwrap here safely because of clap configuration
            let nam = m.value_of("name").unwrap().to_owned();
            let loc = PathBuf::from(loc);

            let conf = muco::config::Config::get()?;
            muco::library::add(conf, nam, loc)
        }
        //TODO: Add library list/status
        _ => unimplemented!(),
    }
}

fn handle_sync(dev: &ArgMatches) -> Result<()> {
    let device_to_sync = dev.value_of("device").map(|s| s.to_owned());
    let library_to_sync = dev.value_of("library").map(|s| s.to_owned());

    let conf = muco::config::Config::get()?;
    let library = muco::library::Library::get(&conf, library_to_sync)?;

    // Take only devices/libraries that are currently online
    let conf = conf.get_online().context(Kind::Unknown)?;
    let devices = muco::device::Device::get(&conf, device_to_sync)?;

    // dbg!(&devices);
    // dbg!(&library);

    muco::sync(&conf, devices, library).context(Kind::Unknown)?;
    Ok(())
}
