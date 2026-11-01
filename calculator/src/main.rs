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


#[derive(Debug)]
struct Stack {
    items: Vec<f64>,
}

impl Stack {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    fn push(&mut self, v: f64) {
        self.items.push(v);
    }

    fn pop(&mut self) -> Option<f64> {
        self.items.pop()
    }

    /// Look at the top number without removing it.
    fn peek(&self) -> Option<&f64> {
        self.items.last()
    }

    fn len(&self) -> usize {
        self.items.len()
    }

    /// Take the two top numbers (b is the top, a is below it), calculate
    /// `a op b` and push the result. We check and calculate first and change
    /// the stack only afterwards, so if there are fewer than two numbers or
    /// the operation fails, the stack stays unchanged and we return None.
    fn apply(&mut self, op: Op) -> Option<()> {
        let n = self.items.len();
        if n < 2 {
            return None;
        }
        let a = self.items[n - 2];
        let b = self.items[n - 1];
        let result = op.apply(a, b)?;
        self.items.pop();
        self.items.pop();
        self.items.push(result);
        Some(())
    }
}

fn main() {
    println!("2 + 3 = {:?}", evaluate_simple(2.0, '+', 3.0));
}