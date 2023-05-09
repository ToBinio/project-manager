mod commands;
mod config;
mod display;
mod project;

use crate::config::Config;
use crate::display::Display;
use clap::builder::Str;
use clap::{command, ArgMatches, Command};
use serde::{Deserialize, Serialize};
use std::{env, fs};
use std::io::Error;

fn main() {
    let mut config = match fs::read_to_string("./Test.toml") {
        Ok(data) => toml::from_str(&data).expect("i dont think so :)"),
        Err(_err) => {
            let config = Config::default();

            fs::write(
                "./Test.toml",
                toml::to_string(&config).expect("could not parse"),
            )
            .expect("could not write file");

            config
        }
    };

    let matches = command!()
        .subcommands(commands::subcommands())
        .get_matches();

    if let Some((cmd, arg)) = matches.subcommand() {
        commands::exec_subcommand(cmd, arg, &mut config);
    } else {
        let display = Display::new(&config);

        display.start();
    }

    fs::write(
        "./Test.toml",
        toml::to_string(&config).expect("could not parse"),
    )
    .expect("could not write file");
}
