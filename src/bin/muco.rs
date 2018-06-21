#[macro_use]
extern crate clap;

use clap::{App, Arg, SubCommand};

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
                                .alias("in"))

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

    println!("{:?}", matches);
}
