use clap::{parser::ValueSource, Arg, Command};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let mut matches = Command::new("headr")
        .version("0.1.0")
        .author("Derek Warner <derekw3@illinois.edu>")
        .about("A rusty head")
        .arg(
            Arg::new("files")
                .help("input files")
                .default_value("-")
                .num_args(1..),
        )
        .arg(
            Arg::new("number_lines")
                .short('n')
                .long("lines")
                .help("number of lines to print")
                .num_args(1)
                .default_value("10")
                .conflicts_with("bytes"),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .help("number of bites to print")
                .num_args(1)
                .default_value("10"),
        );
}
