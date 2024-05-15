// INFO:
// URL: https://adventofcode.com/2015/day/1
// RUN: cargo run -- '()' src/input.txt

use std::env;
use std::fs;
use std::process;

fn main() {
    println!("== I was told there would be no math ==");

    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    let modified_string = count_and_replace_x(&contents);
    println!("Modified String: {}", modified_string);

    // println!("Contents: {}", contents);
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

fn count_and_replace_x(contents: &str) -> String {
    let new_contents: String = contents
        .chars()
        .map(|c| if c == 'x' { '*' } else { c })
        .collect();

    new_contents
}

fn convert_string_signs_to_numbers(new_contents: &str) -> usize {
    todo!();
}

fn calculate_prism_area(new_contents: &str) {
    let modified_string = count_and_replace_x(&new_contents);
}
