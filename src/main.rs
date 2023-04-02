use std::*;

pub mod cli;
pub mod commands;
pub mod utils;

fn main() {
    let app = cli::app();
    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("run", sub)) => commands::run::logic(sub),
        Some((_, _)) => println!("Invalid command"),
        None => println!("Invalid command"),
    };
}
