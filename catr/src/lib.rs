use clap::{parser::ValueSource, Arg, Command};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("Derek Warner <derekw3@illinois.edu>")
        .about("A rusty cat")
        .arg(
            Arg::new("files")
                .help("input files")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("number_lines")
                .short('n')
                .long("number")
                .help("Number lines including blank lines")
                .num_args(0),
        )
        .arg(
            Arg::new("number_nonblank_lines")
                .short('b')
                .long("number-nonblank")
                .help("Number lines excluding blank lines")
                .num_args(0),
        )
        .get_matches();

    let files = matches.get_many::<String>("files").unwrap();

    let files_vec = files.map(|s| s.to_string()).collect();

    let number: bool = matches!(
        matches.value_source("number_lines").unwrap(),
        ValueSource::CommandLine
    );

    let number_nonblank: bool = matches!(
        matches.value_source("number_nonblank_lines").unwrap(),
        ValueSource::CommandLine
    );

    Ok(Config {
        files: files_vec,
        number_lines: number,
        number_nonblank_lines: number_nonblank,
    })
}
