use std::ops::{
    Add,
    Sub,
    Mul,
    Div,
};

#[derive(Debug)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub enum Expression {
    Literal(f64),
    Relationship(Box<Expression>, Operator, Box<Expression>),
}

macro_rules! impl_ops {
    ($varient:ident, $trait:ident, $func:ident) => (
        impl $trait for Expression {
            type Output = Expression;

            fn $func(self, other: Expression) -> Expression {
                Expression::Relationship(
                    Box::new(self),
                    Operator::$varient,
                    Box::new(other)
                ) 
            }
        }
    )
}

impl_ops!(Add, Add, add);
impl_ops!(Subtract, Sub, sub);
impl_ops!(Multiply, Mul, mul);
impl_ops!(Divide, Div, div);

impl Expression {
    pub fn eval(&mut self) -> f64 {
        match self {
            Expression::Literal(n) => *n,
            Expression::Relationship(l, op, r) => {
                let l = l.eval();
                let r = r.eval();
                match op {
                    Operator::Add      => l + r,
                    Operator::Subtract => l - r,
                    Operator::Multiply => l * r,
                    Operator::Divide   => l / r,
                }
            }
        }
    }

    // Handles things like
    // 5 + 1
    // 3

    // fn parse_inner(mut input: String) -> Result<Expression, ParseError> {
    //     let mut segments = input
    //         .chars()
    //         .peekable();

    //     let f1: f64 = next_number(&mut segments).ok_or(ParseError::NoTokens)?;

    //     if let Some(op) = next_op(&mut segments) {
    //         if let Some(f2) = segments.next() {
    //             let op = Operator::parse(op)?;
    //             let f2 = f2.parse().map_err(|_| ParseError::BadNumber)?;
    //             let e1 = Box::new(Expression::Literal(f1));
    //             let e2 = Box::new(Expression::Literal(f2));
    //             return Ok(Expression::Relationship(e1, op, e2))
    //         } else {
    //             return Err(ParseError::MissingTokens);
    //         }
    //     } else {
    //         return Ok(Expression::Literal(f1))
    //     }
    // }
}
