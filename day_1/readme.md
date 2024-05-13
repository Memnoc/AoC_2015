## Day 1 - Not Quite Lisp

This puzzle looked fairly simple in theory, but me being super new to Rust, I found it quite challenging and it toolk me a while to figure it out.

### Puzzle

You are given a series of characters `( = +1` and `) = -1` and they represent steps in an imaginary stair that leads Santa through floors in a building.

**Part I**
The first question to answer was: 'To what floor do the instructions take Santa?'

**Solution**
I went very simple with a solution that gets all the inputs given (a bunch of ()) into a `.txt` file.
I then store the characters in a `Vector` and traverse it, taking note of which character I am currently on.
If the character is `(` I fire a function in a `match` statement that adds 1 to a `counter` varialbe. I do the same exact thing, but adding -1
if the character is `)`.

```rust
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
```

The most difficult part I guess was scope, and how to write the code in terms of syntax. The logic per se was not too bad.

**Part II**
The second question to answer was: 'What is the position of the character that causes Santa to first enter the basement?'

In this part I had few ideas that did not work the way I thought they should. I then ask GPT to give me some help, and the solution it came up with was actually practically the same as the one I have devised, just scoped better.

```rust
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
```

So this function looks a lot like the one I use to count the parenthesis, so much in fact that I thought about this code not being too DRY - however, I like my function to have one responsibility and this approach serves well.

The other bit GPT suggested and that I was struggling with was:

```rust
    match find_first_basement_position(&contents) {
        Some(pos) => println!("Santa is in the basement: {}", pos + 1),
        None => println!("Santa is safe"),
    }
```

So I know about `Option` but I did not know about `Some` or `None`. We use some to represent a successful operation that returns a value, and `None` to represent an operation with an absence of value or failing to reproduce one.

In retrospective, I am a bit distracted because there is a chapter in "The Book" that is title [Matching with Option<T>](https://rust-book.cs.brown.edu/ch06-02-match.html?highlight=Option%3CT%3E#matching-with-optiont) so yeah...
