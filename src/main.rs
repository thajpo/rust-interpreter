use std::io;
use std::io::Write;
use std::str;

fn main() {
    // Get latest on lines buffer
    let mut lines = io::stdin().lines();

    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");

        match lines.next() {
            Some(Ok(input)) => {
                let tokens: Vec<&str> = input.split_whitespace().collect();
                println!("{:?}", tokens)
            }
            _ => break,
        }
    }
}
