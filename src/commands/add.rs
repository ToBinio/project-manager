use crate::Config;
use clap::builder::Str;
use clap::{arg, command, Arg, ArgMatches, Command};

pub fn cli() -> Command {
    Command::new("add").arg(Arg::new("PATH").short('p').long("path"))
}

pub fn exec(arg: &ArgMatches, config: &mut Config) {
    let path = arg
        .get_one::<String>("PATH")
        .expect("no path was entered du pisser!");

    //todo format to corrent path

    config.program_paths.push(path.to_string());
}
