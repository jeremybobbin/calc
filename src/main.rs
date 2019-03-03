use std::env::args; 

pub mod expression;
use expression::*;

pub mod parser;
use parser::*;

#[derive(Debug)]
enum ParseError {
    Operator,
    NoTokens,
    BadNumber,
    MissingTokens
}



fn next_number(chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<f64> {
    next_having(chars, |c| c.is_ascii_digit()).parse().ok()
}

fn next_having<F>(chars: &mut std::iter::Peekable<std::str::Chars>, pred: F) -> String
    where F: Fn(char) -> bool {
    let mut filtered = String::new();
    while let Some(c) = chars.peek() {
        if pred(*c) {
            filtered.push(*c);
        } else if !c.is_ascii_whitespace() {
            break;
        }
        chars.next();
    }
    filtered
}


impl From<f64> for Expression {
    fn from(n: f64) -> Expression {
        Expression::Literal(n)
    }
}

fn main() {
    let mut input: String = String::new();
    for a in args().skip(1) {
        input.push_str(&a);
    }
    let mut input: String = input.split_whitespace()
        .flat_map(|s| s.chars())
        .collect();

    let mut exp_it = ExpressionIter(input.chars().peekable());
    match exp_it.next_expression() {
        Some(mut e) => println!("{}", e.eval()),
        None => panic!("Nigs")
    }
}
