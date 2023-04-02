use crate::commands::run;
use clap::Command;

pub fn app() -> Command {
    Command::new("solana-exercises").subcommand(run::command())
}
