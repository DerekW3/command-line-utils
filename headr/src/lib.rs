use clap::{parser::ValueSource, Arg, Command};
use std::{error::Error, result};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}

pub fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    }
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
        )
        .get_matches();

    let files_vec: Vec<String> = matches.remove_many("files").unwrap().collect();

    let number_lines_flag = matches!(
        matches.value_source("number_lines").unwrap(),
        ValueSource::CommandLine
    );

    let number_bytes_flag = matches!(
        matches.value_source("bytes").unwrap(),
        ValueSource::CommandLine
    );

    let mut number_lines: usize = 0;
    if number_lines_flag {
        let input_number_string: String = matches.remove_one("number_lines").unwrap();
        let input_number_lines = parse_positive_int(&input_number_string);
        match input_number_lines {
            Ok(num) => number_lines = num,
            Err(e) => return Err(e),
        }
    }

    Ok(Config {
        files: files_vec,
        lines: number_lines,
        bytes: Some(10),
    })
}

#[test]
fn test_parse_positive_int() {
    // 3 is valid
    let result = parse_positive_int("3");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 3);

    let result = parse_positive_int("testy");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "testy".to_string());

    // Zero is not valid
    let result = parse_positive_int("0");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "0".to_string());
}
