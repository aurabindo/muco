#[macro_use]
extern crate clap;
extern crate muco;
extern crate walkdir;
extern crate failure;

use clap::{App, Arg, SubCommand} ;
use muco::library::Library;
use muco::device::{Config, Device};
use muco::errors::{Result, MucoError};
use muco::AudFmt;

use std::path::PathBuf;

fn main() {
    let matches = App::new("Muco")
        .about("Stupid Music Copier")
        .author(crate_authors!())
        .version(crate_version!())
        .subcommand(SubCommand::with_name("library")
                    .about("Library management")
                    .alias("lib")
                    .subcommand(SubCommand::with_name("init")
                                .about("Initialize library")
                                .alias("in")
                                .arg(Arg::with_name("exclude")
                                     .short("e")
                                     .long("exclude")
                                     .value_name("DIRS")
                                     .takes_value(true)
                                     .multiple(true)
                                     .help("Exclude directories")))

                    .subcommand(SubCommand::with_name("uninit")
                                .about("Uninitialize/Remove library")
                                .alias("un"))

                    .subcommand(SubCommand::with_name("update")
                                .about("Update existing library")
                                .alias("up")))

        .subcommand(SubCommand::with_name("device")
                    .about("Device management")
                    .alias("dev")
                    .subcommand(SubCommand::with_name("add")
                                .about("Add device")
                                .alias("ad")
                                .arg(Arg::with_name("format")
                                     .short("f")
                                     .long("format")
                                     .value_name("FMT")
                                     .takes_value(true)
                                     .multiple(true)
                                     .requires_all(&["name", "location"])
                                     .help("Device supported formats"))
                                .arg(Arg::with_name("name")
                                     .short("n")
                                     .long("name")
                                     .value_name("NAME")
                                     .takes_value(true)
                                     .multiple(false)
                                     .requires_all(&["format", "location"])
                                     .help("Name for the device"))
                                .arg(Arg::with_name("location")
                                     .short("l")
                                     .long("location")
                                     .value_name("LOC")
                                     .takes_value(true)
                                     .multiple(false)
                                     .requires_all(&["format", "name"])
                                     .help("Mount point of the device")))

                    .subcommand(SubCommand::with_name("remove")
                                .about("Uninitialize device")
                                .alias("re")
                                .arg(Arg::with_name("name")
                                     .short("n")
                                     .long("name")
                                     .value_name("NAME")
                                     .takes_value(true)
                                     .multiple(false)
                                     .required(true)
                                     .help("Name of the device to be removed")))

                    .subcommand(SubCommand::with_name("sync")
                                .about("Synchronize library with device")
                                .alias("sy")
                                .arg(Arg::with_name("name")
                                     .short("n")
                                     .long("name")
                                     .value_name("NAME")
                                     .takes_value(true)
                                     .multiple(false)
                                     .required(false)
                                     .help("Name of the devices to be synchronized with the library")))

                    .subcommand(SubCommand::with_name("status")
                                .about("Report status of all devices known")
                                .alias("st")))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("library") {
        if let Some(matches) = matches.subcommand_matches("init") {
            println!("Detected lib init");

            let exclude_folders = match matches.values_of("exclude") {
                Some(excl) => excl.into_iter().map(|file| String::from(file)).collect::<Vec<_>>(),
                None => vec![],
            };

            Library::init(Some(exclude_folders)).ok();
        }

        if let Some(_matches) = matches.subcommand_matches("uninit") {
            println!("Detected lib uninit");
            let _res = Library::uninit();
        }

        if let Some(_matches) = matches.subcommand_matches("update") {
            let _res = Library::update();
        }
    } else {
        if let Some(matches) = matches.subcommand_matches("device") {
            if let Some(_matches) = matches.subcommand_matches("sync") {
                let names = match matches.values_of("name") {
                    Some(excl) => excl.into_iter().map(|file| String::from(file)).collect::<Vec<_>>(),
                    None => vec![],
                };

                match do_stuff_dev_sync(names) {
                    Ok(_) => println!("Device sync succeeded"),
                    Err(e) => println!("{:?}", e),
                }
            }

            if let Some(_matches) = matches.subcommand_matches("status") {
                match do_stuff_list() {
                    Ok(_) => println!("Report device status succeeded"),
                    Err(e) => println!("{:?}", e),
                }
            }

            if let Some(matches) = matches.subcommand_matches("remove") {
                let name = String::from(matches.value_of("name").unwrap());

                match do_stuff_dev_remove(&name) {
                    Ok(_) => println!("Device removal succeeded"),
                    Err(e) => println!("Could not remove device: {:?}", e),
                }

            }

            if let Some(matches) = matches.subcommand_matches("add") {
                println!("inside add: match : {:?}", matches);

                let formats = match matches.values_of("format") {
                    Some(excl) => excl.into_iter().map(|file| String::from(file)).collect::<Vec<_>>(),
                    None => vec![String::from("mp3")], //use mp3 as default
                };

                // Terrible hack:
                // Unwraps cannot panic as clap wont allow it to reach here
                // if they're not provided by the user.
                let location = PathBuf::from(matches.value_of("location").unwrap());
                let name = String::from(matches.value_of("name").unwrap());

                match do_stuff_dev_add(formats, location, name) {
                    Ok(_) => println!("Succesfully added device to configuration"),
                    Err(e) => {
                        match e {
                            MucoError::DuplicateDevcie => println!("Duplicate device with same parameters exists!"),
                            _ => println!("Error adding device:\n{:?}", e),
                        }
                    }
                }
            }
        }
    }
}

fn do_stuff_dev_sync(n: Vec<String>) -> Result<()> {
    let sys = Config::init()?;

    let devs = sys.devices.iter()
        .filter(|d| (n.len() == 0) || n.contains(&d.name))
        .map(|s| s.clone())
        .collect::<Vec<_>>();

    for dev in devs {
        sys.sync(&dev)?;
    }

    Ok(())
}

fn do_stuff_dev_remove(name: &String) -> Result<()> {
    let mut sys = Config::init()?;

    sys.remove_device(name)?;

    Ok(())
}

fn do_stuff_list() -> Result<()> {
    let sys = Config::init()?;

    sys.list_devices()?;

    Ok(())
}


fn do_stuff_dev_add(fmt: Vec<String>, loc: PathBuf, name: String) -> Result<()> {
    let mut sys: Config = Config::init()?;

    let dev = Device {
        name: name,
        location: loc,
        formats: fmt.into_iter()
            .map(|item| {
                match item.parse::<AudFmt>() {
                    Ok(f) => f,
                    Err(_e) => AudFmt::Mp3 // default is mp3
                }
            })
            .collect::<Vec<_>>(),
    };

    sys.add_device(&dev)?;

    Ok(())
}
