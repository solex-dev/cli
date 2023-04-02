use colored::Colorize;
use std::{path::PathBuf, process::exit};

pub fn does_file_exists(file_path: &String) -> bool {
    PathBuf::from(file_path).exists()
}

pub fn error_message(msg: &str) {
    println!("{}\n", msg.red());
    exit(1);
}

pub fn success_message(msg: &str) {
    println!("{}\n", msg.green());
    exit(0);
}
