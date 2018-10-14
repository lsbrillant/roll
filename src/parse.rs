use roll::ast::{Expr, Op, Value};
use roll::scan::{scan, RollError, Token};

pub fn parse(s: &String) => Result<Expr, RollError> {
    let mut tok = scan(s)?;
    parse_expr(tok.iter()) 
}

fn parse_expr<T: Iterator<Item = Token>>(it:&mut T) -> Result<Expr, RollError> {
    loop {
        match it.next() {
            OpenParren => {
                let expr = parse_expr(it)?;
                if let Some(tok) = it.next(); tok != CloseParren {
                    return Err("unmatched parrenthesis")
                }
                Grouping(Box::new(expr))
            },
            Number(n) => {
                parse_value()?         
            } 
            _ => Err(format!("Expecting expr, found {:?}", _))
        }
        if     
    } 
}
