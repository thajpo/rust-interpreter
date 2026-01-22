use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::str;

const DEBUG: bool = false;

fn main() {
    // Get latest on lines buffer
    let mut lines = io::stdin().lines();
    let interpreter = Interpreter::new();
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");

        match lines.next() {
            Some(Ok(input)) => {
                let tokens = tokenize(&input);
                if DEBUG {
                    println!("{:?}", tokens);
                }

                match parse_tokens(tokens) {
                    Ok(tree) => {
                        println!("{:?}", interpreter.parse_and_evaluate_tree(tree));
                    }
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            }
            _ => break,
        }
    }
}

enum TokenKind {
    StartExpr,
    EndExpr,
    Number(i64),
    Operation(Operations),
    VariableDef,
    FunctionDef,
    Variable(String),
}

// Defines the tree structure we save. Stack pop becomes node
#[derive(Debug)]
enum ExecutionNode {
    Number(i64),
    Call(HeadKind, Vec<ExecutionNode>),
    Let(String, Box<ExecutionNode>),
    Def(String, Box<ExecutionNode>),
    Variable(String),
}

// Defines limit of operations we can perform
#[derive(Debug, PartialEq)]
enum Operations {
    Add,
    Sub,
    Mul,
    Div,
}

// Defines the information we store in the stack
#[derive(Debug)]
struct PartialNode {
    head: Option<HeadKind>,
    args: Vec<ExecutionNode>,
}

#[derive(Debug)]
enum HeadKind {
    BuiltIn(Operations),
    Let,
    Def,
}

/// Turns tokens into instruction steps for interpreter
fn parse_tokens(tokens: Vec<String>) -> Result<ExecutionNode, String> {
    let mut stack: Vec<PartialNode> = vec![];
    let mut result: Option<ExecutionNode> = None;

    for token in tokens {
        let token_type = classify_token(&token);

        match token_type {
            TokenKind::StartExpr => {
                stack.push(PartialNode {
                    head: None,
                    args: vec![],
                });
            }
            TokenKind::EndExpr => {
                let node = stack
                    .pop()
                    .ok_or("Err: Closing parentheses without open.")?;

                let head = node.head.ok_or("Err: No head found.")?;
                let new_node = ExecutionNode::Call(head, node.args);

                if let Some(current) = stack.last_mut() {
                    current.args.push(new_node);
                } else {
                    result = Some(new_node);
                }
            }
            other => {
                let current = stack
                    .last_mut()
                    .ok_or("Err: Tried accessing stack with nothing on it.")?;

                match other {
                    // if we see a number, push
                    TokenKind::Number(n) => current.args.push(ExecutionNode::Number(n)),
                    // if we see a 'def' set head
                    TokenKind::FunctionDef => {
                        current.head = Some(HeadKind::Def);
                    }
                    // if we see an op, set head
                    TokenKind::Operation(op) => {
                        current.head = Some(HeadKind::BuiltIn(op));
                    }
                    // if we see a 'let', set head
                    TokenKind::VariableDef => {
                        current.head = Some(HeadKind::Let);
                    }
                    TokenKind::Variable(var) => current.args.push(ExecutionNode::Variable(var)),
                    _ => unreachable!(),
                }
            }
        }
    }
    result.ok_or("Err: No closing parenthases.".to_string())
}

fn classify_token(token: &str) -> TokenKind {
    if let Some(op) = parse_operations(token) {
        TokenKind::Operation(op)
    } else if let Ok(n) = token.parse::<i64>() {
        TokenKind::Number(n)
    } else if token == "(" {
        TokenKind::StartExpr
    } else if token == ")" {
        TokenKind::EndExpr
    } else if token == "let" {
        TokenKind::VariableDef
    } else if token == "def" {
        TokenKind::FunctionDef
    } else {
        TokenKind::Variable(token.to_string())
    }
}

fn parse_operations(s: &str) -> Option<Operations> {
    match s {
        "+" => Some(Operations::Add),
        "-" => Some(Operations::Sub),
        "*" => Some(Operations::Mul),
        "/" => Some(Operations::Div),
        _ => None,
    }
}

struct Interpreter {
    vars: HashMap<String, i64>,
}

impl Interpreter {
    fn new() -> Self {
        Interpreter {
            vars: HashMap::new(),
        }
    }
    fn get_var(&self, name: &str) -> Option<&i64> {
        self.vars.get(name)
    }
    fn set_var(&mut self, name: String, value: i64) {
        self.vars.insert(name, value);
    }
    fn parse_and_evaluate_tree(&self, tree: ExecutionNode) -> Option<i64> {
        // tree has structure ExecutionNode {Number _or_ Call}
        // Call(op, <nodes>)
        // <nodes> can contain call objects
        // we need to go depth-wise into tree <node>, evaluate and work our way up
        match tree {
            // if number, just return it
            ExecutionNode::Number(n) => Some(n),
            // if var, then get it from hashmap
            ExecutionNode::Variable(var) => Some(self.get_var(&var).copied().unwrap()),
            // if call, then...
            ExecutionNode::Call(head, args) => {
                // values hold our nested call results (recursive calls)
                let mut values: Vec<i64> = vec![];
                if DEBUG {
                    println!("Args: {:?}", args);
                }
                // args are the children of the current node
                // we need to evaluate each child
                for child in args {
                    if DEBUG {
                        println!("Child: {:?}", child);
                    }
                    values.push(self.parse_and_evaluate_tree(child).unwrap());
                }
                // once we have all child values, we can operate on them with the head
                match head {
                    HeadKind::BuiltIn(Operations::Add) => Some(values.iter().sum()),
                    HeadKind::BuiltIn(Operations::Mul) => Some(values.iter().product()),
                    HeadKind::BuiltIn(Operations::Sub) => Some(values[0] - values[1]),
                    HeadKind::BuiltIn(Operations::Div) => Some(values[0] / values[1]),
                }
            }
            ExecutionNode::Let(name, value) => {
                let value = self.parse_and_evaluate_tree(*value).unwrap();
                self.set_var(name, value);
                None
            }
            ExecutionNode::Def(name, value) => {
                let value = self.parse_and_evaluate_tree(*value).unwrap();
                self.set_var(name, value);
                None
            }
        }
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
