use clap::{Command, command};

fn main() {
    let matches =
        command!().
        get_matches();

    println!("{:?}", matches);
}
