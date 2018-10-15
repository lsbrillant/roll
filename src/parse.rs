use ast::{Expr, Op, Value};
use scan::{RollError, Token};

pub fn parse_expr<T: Iterator<Item = Token>>(it:&mut T) -> Result<Expr, RollError> 
    where T: Clone {
    if let Some(tok) = it.next() {
        let ex = match tok {
            Token::OpenParren => {
                let expr = parse_expr::<T>(it.by_ref())?;
                if let Some(Token::CloseParren) = it.next() {
                    Expr::Grouping(Box::new(expr))
                } else {
                    return Err("unmatched parrens".to_string()) 
                }
            },
            Token::Number(n) => match it.clone().peekable().peek() {
                Some(Token::D) => {
                    it.next();// the "D"
                    match it.next() {
                        Some(Token::Number(sides)) => Expr::Literal(Box::new(Value::Dice(n, sides))),
                        _ => return Err("a \"d\" must have a number after it".to_string())
                    }
                },
                _ => {
                    Expr::Literal(Box::new(Value::Number(n)))
                }
            }, 
            _ => return Err(format!("Expecting expr, found {:?}", tok).to_string())
        };
        if let Some(n) = it.clone().peekable().next() {
            Ok(match n {
                Token::AddOp => {
                    it.next();
                    Expr::Binary(
                        Box::new(ex),
                        Op::Add,
                        Box::new(parse_expr::<T>(it.by_ref())?)
                        )
                },
                Token::SubOp => {
                    it.next();
                    Expr::Binary(
                        Box::new(ex),
                        Op::Sub,
                        Box::new(parse_expr::<T>(it.by_ref())?)
                        )
                },
                _ => ex,
            })
        } else {
            Ok(ex)
        }
    } else {
        Err("can't parse empty string".to_string()) 
    } 
}

#[cfg(test)]
mod test {
    use scan::{Token, Scanner};
    use ast::{Expr, Value, Op};

    #[test]
    fn test_simple_parse() {
        let expected = Expr::Binary(
            Box::new(Expr::Literal(Box::new(Value::Dice(1, 4)))),
            Op::Add,
            Box::new(Expr::Literal(Box::new(Value::Number(1))))
            );
        if let Ok(result) = super::parse_expr(&mut Scanner::new("1d4+1".chars())) {
            assert!(result == expected, "{:?}", result);
        } else {
            assert!(false);
        }
    }
}
