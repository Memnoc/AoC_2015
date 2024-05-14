# Day 2 - I was told there would be no math

This one has a little geometry in it.

1. You have to calculate the area of a rectangular prism.

`A=2(l*w+l*h+w*h)`

2. The other requirement is calculating the smallest area of the trapezoid for extra packaging;

For example:

```
length = 3
widht = 4
height = 5
```

`A = 2(3 x 4 + 3 x 5 + 4 x 5) = 2(12 + 15 + 20) = 2 x 47 = 94 square meters`

To get the second requirement, you only need to take the smallest area value, in the example `12`

You are given a list of of boxes, as such:
`28x9x8`

The meat of the puzzle is to process the data from the file and sum up the results, as the very core of the challenge is:
"How many total square feet of wrapping paper should they order?"

**Solution**
The first thing to do is to parse the data from the files. I think we can use the same motion we have used in the previous challenge and put everything in a `Vector`:

_in main.rs_

```rust
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");
```

Then we create and implement the `Struct` for `Config`:

```rust
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
```

This allow us to read from the file, and all the entries are printed as strings, which is a problem. So the next problem to solve is to convert the output back into numbers.
To solve it, we could create a function that converts the string back into numbers, but before doing so, there is the problem of how computing them into mathematical operations.
We have an output that is for example `8x11x6` so we could take the advantage of the format and the sign `x` - we could write a function that permutes all the `x`s into `*` and then pass each character to the function that performs the area calculation.

So to sum up:

1. Put all hints in a file
2. Read all inputs in the file
3. Change all occurrences of `x` to `*`
   3.1 for example `8x11x6` becomes `8*11*6`
4. Convert the string into a number
5. Pass the value to the `calculate_prism_area` function
6. Inside `calculate_prism_area` call `add_extra_slack`
   6.1 this function takes the smallest area as parameter and adds the result to the `total` square feet of wrapping paper
