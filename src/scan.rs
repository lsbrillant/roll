#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(i32),
    D,
    AddOp,
    SubOp,
    OpenParren,
    CloseParren,
}

type RollError = String;


pub fn scan(s: &String) -> Result<Vec<Token>, RollError> {
    let mut tokens = Vec::new();

    let mut it = s.chars().peekable();
    while let Some(&ch) = it.peek() {
        match ch {
            ' ' => {
                it.next();
            }
            'd' | 'D' => {
                it.next(); // The "D"
                tokens.push(Token::D);
            }
            '0'...'9' => {
                let n = scan_number(&mut it)?;
                tokens.push(Token::Number(n));
            }
            '+' => {
                it.next();
                tokens.push(Token::AddOp);
            }
            '-' => {
                it.next();
                tokens.push(Token::SubOp);
            }
            '(' => {
                it.next();
                tokens.push(Token::OpenParren);
            }
            ')' => {
                it.next();
                tokens.push(Token::CloseParren);
            }
            _ => {
                it.next();
            }
        }
    }

    Ok(tokens)
}
fn scan_number<T: Iterator<Item = char>>(it: &mut T) -> Result<i32, RollError> 
    where T: Clone {
    let num_str = it.clone().take_while(|c| c.is_digit(10)).collect::<String>();
    // Just a little bit of a hack to get the iterator not to be wrong,
    // maybe I will change it latter.
    for _ in num_str.chars() {
        it.next();
    } 
    match num_str.parse::<i32>()
    {
        Ok(n) => Ok(n),
        Err(err) => Err(format!("{}", err)),
    }
}


#[cfg(test)]
mod test {
    use super::scan;
    use super::Token::*;
    
    #[test]
    fn test_simple_scan() {
        let line = "1d4 + 1".to_string();
        let expected = vec![Number(1), D, Number(4), AddOp, Number(1)];
        if let Ok(result) = scan(&line) {
            for (i, item) in expected.iter().enumerate() {
                assert!(result[i] == *item);
            }
        } else {
            assert!(false);
        }
    }
}
