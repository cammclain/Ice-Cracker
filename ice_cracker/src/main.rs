use clap::{Arg, App};
use polars::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;
use std::fs::File;
use std::process;
use md5;
use phpass;
use bcrypt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;
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

    // Intake id:password_hash from a txt file, where each line is that combo. In this case "id" may be an email, or a username.
    let id_password_path = "path/to/id_password.txt";
    let id_password_df = read_id_passwords(id_password_path)?;

    // Intake dictionary words from a txt file, where each line is a word.
    let dict_words_path = "path/to/dictionary.txt";
    let dict_words_df = read_dictionary_words(dict_words_path)?;

    // Here, add your logic for the hashing and checking process

    Ok(())
}

fn read_id_passwords(path: &str) -> PolarsResult<DataFrame> {
    let file = File::open(path).unwrap_or_else(|_| {
        eprintln!("Failed to open file: {}", path);
        process::exit(1);
    });
    let id_password_df = BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            let mut split = line.split(':');
            let id = split.next().unwrap();
            let password = split.next().unwrap();
            Series::new("id", vec![id.to_string()])
                .append(&Series::new("password", vec![password.to_string()])) // Fixed: Borrow the Series
        })
        .collect::<Vec<_>>();
    DataFrame::new(id_password_df)
}

fn read_dictionary_words(path: &str) -> PolarsResult<DataFrame> {
    let file = File::open(path).unwrap_or_else(|_| {
        eprintln!("Failed to open file: {}", path);
        process::exit(1);
    });
    let dict_words_df = BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok)
        .map(|line| Series::new("words", vec![line]))
        .collect::<Vec<_>>();
    DataFrame::new(dict_words_df)
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



use std::collections::HashMap;

fn perform_attack(dict_df: DataFrame, hash_df: DataFrame) -> HashMap<String, String> {
    let mut results = HashMap::new();
    let dictionary = dict_df.column("words").unwrap().utf8().unwrap();
    let hashes = hash_df.column("hashes").unwrap().utf8().unwrap();

    for word in dictionary {
        let hash = compute_hash(word); // This function needs to be implemented based on the hash type
        if hashes.contains(&hash) {
            results.insert(hash, word.to_string());
        }
    }

    results
}

fn compute_hash(word: &str) -> String {
    // Implement the hashing algorithm here. For example, using MD5:
    format!("{:x}", md5::compute(word))
}