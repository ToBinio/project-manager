use crate::config::Config;
use clap::{ArgMatches, Command};

mod add;

pub fn subcommands() -> Vec<Command> {
    vec![add::cli()]
}

pub fn exec_subcommand(cmd: &str, arg: &ArgMatches, config: &mut Config) {
    match cmd {
        "add" => add::exec(arg, config),
        _ => {}
    }
}
