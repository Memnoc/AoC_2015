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

    let mut total_area = 0;
    let mut total_ribbon = 0;
    for line in contents.lines() {
        match parse_and_calculate_area(line) {
            Ok((area, ribbon)) => {
                total_area += area;
                total_ribbon += ribbon;
            }
            Err(err) => println!("Error calculating area: {}", err),
        }
    }
    println!("Total Area: {}", total_area);
    println!("Total Ribbon: {}", total_ribbon);
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

fn parse_and_calculate_area(line: &str) -> Result<(usize, usize), &'static str> {
    let dimensions: Vec<&str> = line.split('x').collect();
    if dimensions.len() != 3 {
        return Err("Invalid dimensions");
    }

    let l: usize = dimensions[0]
        .parse()
        .map_err(|_| "Invalid number for length")?;
    let w: usize = dimensions[1]
        .parse()
        .map_err(|_| "Invalid number for width")?;
    let h: usize = dimensions[2]
        .parse()
        .map_err(|_| "Invalid number for height")?;

    let lw = l * w;
    let wh = w * h;
    let hl = h * l;

    let surface_area = 2 * lw + 2 * wh + 2 * hl;
    let smallest_side = lw.min(wh).min(hl);
    let total_area = surface_area + smallest_side;

    // Calculate ribbon
    let perimeter1 = 2 * (l + w);
    let perimeter2 = 2 * (w + h);
    let perimeter3 = 2 * (h + l);

    let smallest_perimeter = perimeter1.min(perimeter2).min(perimeter3);
    let volume = l * w * h;

    let total_ribbon = smallest_perimeter + volume;

    println!("total: {}", total_area);

    Ok((total_area, total_ribbon))
}
