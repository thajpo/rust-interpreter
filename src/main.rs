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
                let tokens = tokenize(&input);
                println!("{:?}", tokens);
                let interpreter_blocks = parse_tokens(tokens);
            }
            _ => break,
        }
    }
}

/// Turns tokens into instruction steps for interpreter_blocks
fn parse_tokens(tokens: Vec<String>) -> Vec<TokenType> {
    // 1. if "(" then we will have an operation applied to all subsequent elements until closure
    // 2. if ")" then the closest "("s operatoin no longer applies
    for token in tokens {
        if token = "( " {
            
        }
        else if token = " )" {
            
        }
        else if token {

        }
    }
}

/// Parses line from stdout and splits by whitespace
fn tokenize(input: &str) -> Vec<String> {
    input
        .replace("(", "( ")
        .replace(")", " )")
        .replace(" (", "( ")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

enum Expression {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_simple() {
        let result = tokenize("(+ 1 2)");
        assert_eq!(result, vec!["(", "+", "1", "2", ")"]);
    }

    #[test]
    fn tokenize_nested() {
        let result = tokenize("(+ 1 (* 2 3))");
        assert_eq!(result, vec!["(", "+", "1", "(", "*", "2", "3", ")", ")"]);
    }
}
