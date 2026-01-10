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
                match parse_tokens(tokens) {
                    Ok(tree) => println!("{:?}", tree),
                    Err(e) => println!("{}", e),
                }
            }
            _ => break,
        }
    }
}

fn interpret_tree(tree: ExecutionNode) {}

// Defines limit of operations we can perform
#[derive(Debug)]
enum Operations {
    Add,
    Sub,
    Mul,
    Div,
}

// Defines the information we store in the stack
#[derive(Debug)]
struct PartialNode {
    op: Option<Operations>,
    args: Vec<ExecutionNode>,
}

// Defines the tree structure we save. Stack pop becomes node
#[derive(Debug)]
enum ExecutionNode {
    Number(i64),
    Call(Operations, Vec<ExecutionNode>),
}
/// Turns tokens into instruction steps for interpreter_blocks
fn parse_tokens(tokens: Vec<String>) -> Result<ExecutionNode, String> {
    let mut stack: Vec<PartialNode> = vec![];
    let mut result: Option<ExecutionNode> = None;

    for token in tokens {
        match token.as_str() {
            "(" => {
                // Add partial node if new expression begins
                stack.push(PartialNode {
                    op: None,
                    args: vec![],
                });
            }
            ")" => {
                // Remove partial node when expression ends, add finished node to tree
                let node = stack.pop().unwrap();
                let complete = ExecutionNode::Call(node.op.unwrap(), node.args);

                if let Some(parent) = stack.last_mut() {
                    parent.args.push(complete);
                } else {
                    result = Some(complete);
                }
            }
            _ => {
                // Handling operations, numbers, and errors
                // If token returns operation, then we add the Op to the stack
                if let Some(op) = pase_operations(&token) {
                    stack.last_mut().unwrap().op = Some(op);
                }
                // If token returns a number, then we add to the PartialNode
                if let Ok(n) = token.parse::<i64>() {
                    stack
                        .last_mut()
                        .unwrap()
                        .args
                        .push(ExecutionNode::Number(n));
                }
            }
        }
    }
    result.ok_or("Closing parentheses not used, or other error".to_string())
}

fn pase_operations(s: &str) -> Option<Operations> {
    match s {
        "+" => Some(Operations::Add),
        "-" => Some(Operations::Sub),
        "*" => Some(Operations::Mul),
        "/" => Some(Operations::Div),
        _ => None,
    }
}

/// Parses line from stdout and splits by whitespace
fn tokenize(input: &str) -> Vec<String> {
    input
        .replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
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
