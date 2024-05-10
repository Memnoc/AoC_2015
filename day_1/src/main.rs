// INFO:
// URL: https://adventofcode.com/2015/day/1
// RUN: cargo run -- '()' src/input.txt

use std::env;
use std::fs;
use std::process;

fn main() {
    println!("== Day 1: Not Quite Lisp ==");

    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    // counting instances
    let count = count_partentheses(&contents);
    println!("Total '()' characters: {}", count);

    match find_first_basement_position(&contents) {
        Some(pos) => println!("Santa is in the basement: {}", pos + 1),
        None => println!("Santa is safe"),
    }
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

fn increment(n: &mut isize) {
    *n += 1
}

fn decrement(n: &mut isize) {
    *n -= 1
}

fn count_partentheses(contents: &str) -> isize {
    let mut counter = 1;
    contents
        .chars()
        .enumerate()
        .for_each(|(_index, character)| {
            // println!("{}: {}", _index, character);
            match character == ')' {
                true => {
                    increment(&mut counter);
                }
                false => {
                    decrement(&mut counter);
                }
            }
        });
    counter
}

fn find_first_basement_position(contents: &str) -> Option<usize> {
    let mut counter = 0;
    for (index, character) in contents.chars().enumerate() {
        match character {
            '(' => increment(&mut counter),
            ')' => decrement(&mut counter),
            _ => {}
        }

        if counter == -1 {
            return Some(index);
        }
    }
    None
}
