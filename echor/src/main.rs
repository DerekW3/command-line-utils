use clap::{Arg, Command};

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("Derek Warner <derekw3@illinois.edu>")
        .about("Rust echo implementation")
        .arg(
            Arg::new("text")
                .help("Input text")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print newline")
                .num_args(0),
        )
        .get_matches();

    println!("{:#?}", matches);
}
