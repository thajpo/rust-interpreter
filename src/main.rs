use std::collections::HashMap;
use std::hash::Hash;
use std::io;
use std::io::Write;
use std::str;

const DEBUG: bool = false;

// Defines limit of operations we can perform
#[derive(Debug, PartialEq)]
enum Operations {
    Add,
    Sub,
    Mul,
    Div,
    FuncDef,
}

enum TokenKind {
    StartExpr(String),
    EndExpr(String),
    Number(i64),
    Operation(Operations),
    Variable(String),
    Function(String),
}

// Defines the information we store in the stack
#[derive(Debug)]
struct PartialNode {
    var: Option<String>,
    func: Option<String>,
    op: Option<Operations>,
    args: Vec<ExecutionNode>,
}

// Defines the tree structure we save. Stack pop becomes node
#[derive(Debug)]
enum ExecutionNode {
    Number(i64),
    Call(Operations, Vec<ExecutionNode>),
    Variable(String),
}

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
    fn parse_and_evaluate_tree(&self, tree: ExecutionNode) -> i64 {
        // tree has structure ExecutionNode {Number _or_ Call}
        // Call(op, <nodes>)
        // <nodes> can contain call objects
        // we need to go depth-wise into tree <node>, evaluate and work our way up
        match tree {
            ExecutionNode::Number(n) => n,
            ExecutionNode::Variable(var) => self.get_var(&var).copied().unwrap(),
            ExecutionNode::Call(op, args) => {
                let mut values: Vec<i64> = vec![];
                if DEBUG {
                    println!("Args: {:?}", args);
                }
                for child in args {
                    if DEBUG {
                        println!("Child: {:?}", child);
                    }
                    values.push(self.parse_and_evaluate_tree(child));
                }
                match op {
                    Operations::Add => values.iter().sum(),
                    Operations::Mul => values.iter().product(),
                    Operations::Sub => values[0] - values[1],
                    Operations::Div => values[0] / values[1],
                    Operations::FuncDef => {
                        // If our operation is to define a function, we need to tell the
                        todo!()
                    }
                }
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

/// Turns tokens into instruction steps for interpreter
fn parse_tokens(tokens: Vec<String>) -> Result<ExecutionNode, String> {
    let mut stack: Vec<PartialNode> = vec![];
    let mut result: Option<ExecutionNode> = None;

    for token in tokens {
        let token_type = classify_token(&token);

        match token_type {
            TokenKind::StartExpr(_) => {
                stack.push(PartialNode {
                    var: None,
                    func: None,
                    op: None,
                    args: vec![],
                });
            }
            TokenKind::EndExpr(_) => {
                let node = stack
                    .pop()
                    .ok_or("Err: Closing parentheses without open.")?;

                let complete = match (node.op, node.var) {
                    (Some(op), _) => ExecutionNode::Call(op, node.args),
                    (_, Some(var)) => ExecutionNode::Variable(var),
                    (None, None) => {
                        return Err("Err: No operation or variable provided.".to_string());
                    }
                };

                // check : are we done?
                if let Some(parent) = stack.last_mut() {
                    parent.args.push(complete);
                } else {
                    result = Some(complete);
                }
            }
            other => {
                let current = stack
                    .last_mut()
                    .ok_or("Err: Tried accessing stack with nothing on it.")?;

                match other {
                    TokenKind::Number(n) => current.args.push(ExecutionNode::Number(n)),
                    TokenKind::Function(func) => current.func = Some(func),
                    TokenKind::Operation(op) => current.op = Some(op),
                    TokenKind::Variable(var) => current.var = Some(var),
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
        TokenKind::StartExpr(token.to_string())
    } else if token == ")" {
        TokenKind::EndExpr(token.to_string())
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
        "let" => Some(Operations::FuncDef),
        _ => None,
    }
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
