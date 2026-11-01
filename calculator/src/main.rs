//! A small calculator core: operators and a stack of numbers.
#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    /// Turn a character into an operator, or None if it is not one.
    fn from_char(c: char) -> Option<Op> {
        match c {
            '+' => Some(Op::Add),
            '-' => Some(Op::Sub),
            '*' => Some(Op::Mul),
            '/' => Some(Op::Div),
            _ => None,
        }
    }

    /// The opposite direction: operator -> character.
    fn symbol(self) -> char {
        match self {
            Op::Add => '+',
            Op::Sub => '-',
            Op::Mul => '*',
            Op::Div => '/',
        }
    }

    /// 1 for + and -, 2 for * and /.
    fn precedence(self) -> u8 {
        match self {
            Op::Add => 1,
            Op::Sub => 1,
            Op::Mul => 2,
            Op::Div => 2,
        }
    }

    /// Do the calculation. None when dividing by zero.
    fn apply(self, a: f64, b: f64) -> Option<f64> {
        match self {
            Op::Add => Some(a + b),
            Op::Sub => Some(a - b),
            Op::Mul => Some(a * b),
            Op::Div => {
                if b == 0.0 {
                    None
                } else {
                    Some(a / b)
                }
            }
        }
    }
}

/// If the character is not an operator, `?` returns None right away.
fn evaluate_simple(a: f64, op: char, b: f64) -> Option<f64> {
    Op::from_char(op)?.apply(a, b)
}

fn main() {
    println!("2 + 3 = {:?}", evaluate_simple(2.0, '+', 3.0));
}