use std::ops;
use std::iter::Peekable;
use std::str::Chars;

use crate::expression::*;

type PeekChars<'a> = Peekable<Chars<'a>>;

pub struct ExpressionIter<'a>(pub PeekChars<'a>);

impl<'a> ops::Deref for ExpressionIter<'a> {
    type Target = PeekChars<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> ops::DerefMut for ExpressionIter<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> ExpressionIter<'a> {
    pub fn next_matching<F>(&mut self, pred: F) -> String 
        where F: Fn(char) -> bool 
    {
        let mut sequence = String::new();
        while let Some(&c) = self.peek() {
            if !pred(c) {
                break;
            }
            sequence.push(c);
            self.next();

        }
        sequence
    }

    pub fn next_number(&mut self) -> Option<f64> {
        self.next_matching(|c| c.is_ascii_digit() || c == '.').parse().ok()
    }

    pub fn next_operator(&mut self) -> Option<Operator> {
        let c = *self.peek()?;
        self.next();
        match c {
            '+' => Some(Operator::Add),
            '-' => Some(Operator::Subtract),
            '*' => Some(Operator::Multiply),
            '/' => Some(Operator::Divide),
            _   => None
        }
    }

    pub fn next_expression(&mut self) -> Option<Expression> {
        let l = Expression::Literal(self.next_number()?);
        if let Some(op) = self.next_operator() {
            let l = Box::new(l);
            let r = Box::new(Expression::Literal(self.next_number()?));
            Some(Expression::Relationship(l, op, r))
        } else {
            Some(l)
        }
    }
}
