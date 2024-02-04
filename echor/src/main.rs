use clap::{parser::ValueSource, Arg, Command};

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

    let omit_newline = matches.value_source("omit_newline").unwrap();

    let ending = match omit_newline {
        ValueSource::CommandLine => "",
        _ => "\n",
    };

    if let Some(text) = matches.get_many::<String>("text") {
        let text_vec: Vec<String> = text.map(|s| s.to_string()).collect();
        let joined_text = text_vec.join(" ");

        print!("{}{}", joined_text, ending);
    }
}
