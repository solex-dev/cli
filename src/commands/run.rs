use crate::utils::{does_file_exists, error_message};
use clap::{Arg, ArgMatches, Command};
use colored::Colorize;
use spinoff::{spinners, Color, Spinner};
use std::process;

pub fn command() -> Command {
    Command::new("run")
        .arg(
            Arg::new("exercise")
                .short('e')
                .required(true)
                .help("ID of the exercise"),
        )
        .arg(
            Arg::new("keypair")
                .short('k')
                .required(true)
                .help("Path of the payer's keypair JSON file"),
        )
        .arg(
            Arg::new("program")
                .short('p')
                .required(true)
                .help("Path of the program's keypair JSON file"),
        )
}

pub fn logic(args: &ArgMatches) {
    let exercise: &String = args.get_one("exercise").unwrap();
    let keypair: &String = args.get_one("keypair").unwrap();
    let program: &String = args.get_one("program").unwrap();

    if !does_file_exists(&format!("tests/{}.test.ts", exercise)) {
        error_message("Invalid exercise ID");
    }

    if !does_file_exists(keypair) {
        error_message("Missing keypair file")
    }

    if !does_file_exists(program) {
        error_message("Missing program's keypair file")
    }

    let spinner = Spinner::new(
        spinners::Dots,
        format!("Running test file for {} exercise", exercise),
        Color::Yellow,
    );

    let output = process::Command::new("pnpm")
        .arg("vitest")
        .arg("run")
        .arg(&exercise)
        .env("KEYPAIR", keypair)
        .env("PROGRAM_PATH", program)
        .output();

    match output {
        Ok(output) => {
            let status = output.status;

            if status.success() {
                spinner.success(format!("{}", "The test ran successfully!".green()).as_str());
            } else {
                let error = String::from_utf8(output.stderr).unwrap();

                spinner.stop_with_message(
                    format!(
                        "{}\nError: {}",
                        "An error occcured while running the test".red(),
                        error
                    )
                    .as_str(),
                );
            }
        }
        Err(err) => {
            spinner.stop_with_message(
                format!(
                    "{}\nError: {}",
                    "An error occcured while running the test".red(),
                    err
                )
                .as_str(),
            );
        }
    }
}
