extern crate rand;

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Add, Sub
}
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Box<Value>),
    Grouping(Box<Expr>),
    Binary(Box<Expr>, Op, Box<Expr>),
}
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    // first value is how many, second is the number of sides
    Dice(i32, i32),
    // just a regular number
    Number(i32)
}

pub trait Eval {
    fn eval(&self) -> i32;
}

impl Eval for Value {
    fn eval(&self) -> i32 {
        use self::Value::*;
        match self {
            Dice(num, sides) => {
                use ast::rand::distributions::{Distribution, Uniform};
                let between = Uniform::new(1, *sides+1);
                let mut rng = rand::thread_rng();
                let mut sum = 0;
                for _ in 0..*num {
                    sum += between.sample(&mut rng); 
                }
                sum
            },
            Number(n) => *n
        }
    }
}

impl Eval for Expr {
    fn eval(&self) -> i32 {
        use self::Expr::*;
        match self {
            Literal(lit) => lit.eval(),
            Grouping(expr) => expr.eval(),
            Binary(lhs, op, rhs) => {
                match op {
                    Op::Add => {
                        lhs.eval() + rhs.eval()
                    },
                    Op::Sub => {
                        lhs.eval() - rhs.eval()
                    }
                }
            },
        }
    } 
}

#[cfg(test)]
mod test {
    use super::Eval;
    use super::Expr::*;
    use super::Value::*;

    #[test]
    fn test_simple_const_add() {
        let ast = Binary(
            Box::new(Literal(Box::new(Number(1)))),
            super::Op::Add,
            Box::new(Literal(Box::new(Number(1))))
            );
        assert!(ast.eval() == 2);
    }
    #[test]
    fn test_simple_rand() {
        let ast = Literal(Box::new(Dice(1,4)));
        let val = ast.eval(); 
        assert!(val > 0 && val <= 4);
    }
}
