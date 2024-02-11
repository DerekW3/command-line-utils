use clap::{parser::ValueSource, Arg, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(e) => eprintln!("Failed to open {}: {}", filename, e),
            Ok(file) => {
                println!("Opened {}", filename);

                let mut line_number = 1;
                for line in file.lines() {
                    let line_unwrapped = line.unwrap();
                    if config.number_lines {
                        println!("{}  {}", line_number, line_unwrapped);
                        line_number += 1;
                    } else if config.number_nonblank_lines {
                        if line_unwrapped.is_empty() {
                            println!("{}", line_unwrapped);
                        } else {
                            println!("{}  {}", line_number, line_unwrapped);
                            line_number += 1;
                        }
                    } else {
                        println!("{}", line_unwrapped);
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn get_args() -> MyResult<Config> {
    let mut matches = Command::new("catr")
        .version("0.1.0")
        .author("Derek Warner <derekw3@illinois.edu>")
        .about("A rusty cat")
        .arg(
            Arg::new("files")
                .help("input files")
                .default_value("-")
                .num_args(1..),
        )
        .arg(
            Arg::new("number_lines")
                .short('n')
                .long("number")
                .help("Number lines including blank lines")
                .num_args(0)
                .conflicts_with("number_nonblank_lines"),
        )
        .arg(
            Arg::new("number_nonblank_lines")
                .short('b')
                .long("number-nonblank")
                .help("Number lines excluding blank lines")
                .num_args(0),
        )
        .get_matches();

    let files_vec: Vec<String> = matches.remove_many("files").unwrap().collect();

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
