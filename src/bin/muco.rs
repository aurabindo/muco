#[macro_use]
extern crate clap;
extern crate muco;
extern crate walkdir;

use clap::{App, Arg, SubCommand} ;
use muco::library::Library;

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
                    .subcommand(SubCommand::with_name("init")
                                .about("Initialize device")
                                .alias("in"))

                    .subcommand(SubCommand::with_name("uninit")
                                .about("Uninitialize device")
                                .alias("un"))

                    .subcommand(SubCommand::with_name("sync")
                                .about("Synchronize library with device")
                                .alias("sy")))

        .get_matches();

    if let Some(matches) = matches.subcommand_matches("library") {
        if let Some(matches) = matches.subcommand_matches("init") {
            println!("Detected lib init");

            let exclude_folders = match matches.values_of("exclude") {
                Some(excl) => excl.into_iter().map(|file| String::from(file)).collect::<Vec<_>>(),
                None => vec![],
            };

            // let exclude_folders = matches.values_of("exclude").unwrap()

            println!("something is : {:?}", exclude_folders);
            Library::init(Some(exclude_folders)).unwrap();
        }

        if let Some(_matches) = matches.subcommand_matches("uninit") {
            println!("Detected lib uninit");
            let _res = Library::uninit();
        }

        if let Some(_matches) = matches.subcommand_matches("update") {
            println!("Detected lib update");
            let _res = Library::update();
        }
    } else {
        if let Some(_matches) = matches.subcommand_matches("device") {
            if let Some(_matches) = matches.subcommand_matches("init") {
                println!("Detected dev init");
            }

            if let Some(_matches) = matches.subcommand_matches("uninit") {
                println!("Detected dev uninit");
            }

            if let Some(_matches) = matches.subcommand_matches("sync") {
                println!("Detected dev sync");
            }
        }
    }
}
