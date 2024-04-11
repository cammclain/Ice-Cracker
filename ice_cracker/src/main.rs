use clap::{Arg, App};
use polars::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

fn main() -> PolarsResult<()> {
    let matches = App::new("Ice Cracker")
        .version("0.1")
        .author("Cam Mclain <cammclain@protonmail.com>")
        .about("Cracks passwords using a dictionary attack")
        .arg(Arg::with_name("hashes")
             .short('h')
             .long("hashes")
             .value_name("FILE")
             .help("Sets the input file with password hashes")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("dictionary")
             .short('d')
             .long("dictionary")
             .value_name("FILE")
             .help("Sets the input file with dictionary words")
             .takes_value(true)
             .required(true))
        .get_matches();

    let hash_path = matches.value_of("hashes").unwrap();
    let dict_path = matches.value_of("dictionary").unwrap();

    let hash_df = read_hashes(hash_path)?;
    let dict_df = read_dictionary(dict_path)?;

    // Here, add your logic for the hashing and checking process

    // TODO: Intake id:password_hash from a txt file, where each line is that combo. In this case "id" may be an email, or a username.
    // TODO: Intake dictionary words from a txt file, where each line is a word.
    
    
    Ok(())
}

fn read_hashes(path: &str) -> PolarsResult<DataFrame> {
    let file = File::open(path).unwrap_or_else(|_| {
        eprintln!("Failed to open file: {}", path);
        process::exit(1);
    });
    let hash_df = BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| Series::new("hashes", vec![line]))
        .collect::<Vec<_>>();
    DataFrame::new(hash_df)
}

fn read_dictionary(path: &str) -> PolarsResult<DataFrame> {
    let file = File::open(path).unwrap_or_else(|_| {
        eprintln!("Failed to open file: {}", path);
        process::exit(1);
    });
    let dict_df = BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| Series::new("words", vec![line]))
        .collect::<Vec<_>>();
    DataFrame::new(dict_df)
}
