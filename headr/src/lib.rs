use clap::{parser::ValueSource, Arg, Command};
use std::fs::File;
use std::{
    error::Error,
    io::{self, BufRead, BufReader, Read},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn run(config: Config) -> MyResult<()> {
    let mut file_count = 0;
    let num_files = config.files.len();
    for filename in config.files {
        match open(&filename) {
            Err(e) => eprintln!("{}: {}", filename, e),
            Ok(mut file) => {
                if file_count != 0 {
                    println!();
                }
                if num_files > 1 {
                    println!("==> {} <==", filename);
                }
                match config.bytes {
                    Some(bytes) => {
                        let mut handle = file.take(bytes as u64);
                        let mut buffer = vec![0; bytes];
                        let bytes_read = handle.read(&mut buffer)?;

                        print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                    }
                    None => {
                        let mut line = String::new();
                        for _ in 0..config.lines {
                            let bytes = file.read_line(&mut line)?;
                            if bytes == 0 {
                                break;
                            }
                            print!("{}", line);
                            line.clear();
                        }
                    }
                }
                file_count += 1;
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
                .help("Input file(s)")
                .default_value("-")
                .num_args(1..),
        )
        .arg(
            Arg::new("number_lines")
                .short('n')
                .long("lines")
                .value_name("LINES")
                .help("Number of lines")
                .num_args(1)
                .default_value("10"),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .value_name("BYTES")
                .help("Number of bytes")
                .num_args(1)
                .default_value("10")
                .conflicts_with("number_lines"),
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

    let mut number_lines: usize = 10;
    if number_lines_flag {
        let input_number_string: String = matches.remove_one("number_lines").unwrap();
        let input_number_lines = parse_positive_int(&input_number_string);
        match input_number_lines {
            Ok(num) => number_lines = num,
            Err(e) => return Err(e).map_err(|e| format!("illegal line count -- {}", e))?,
        }
    }

    let mut number_bytes: Option<usize> = None;
    if number_bytes_flag {
        let input_bytes_string: String = matches.remove_one("bytes").unwrap();
        let input_number_bytes = parse_positive_int(&input_bytes_string);
        match input_number_bytes {
            Ok(num) => number_bytes = Some(num),
            Err(e) => return Err(e).map_err(|e| format!("illegal byte count -- {}", e))?,
        }
    }

    Ok(Config {
        files: files_vec,
        lines: number_lines,
        bytes: number_bytes,
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
