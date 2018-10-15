#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(i32),
    D,
    AddOp,
    SubOp,
    OpenParren,
    CloseParren,
}

pub type RollError = String;

#[derive(Debug, Clone)]
pub struct Scanner<I: Iterator<Item=char>> {
    iter: I,
}
impl<I: Iterator<Item=char>> Scanner<I> {
    pub fn new(it: I) -> Scanner<I> {
        Scanner{ iter: it } 
    }
}

impl<I: Iterator<Item=char>> Iterator for Scanner<I> where I: Clone {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(ch) = self.iter.next() {
            match ch { 
                'd' => return Some(Token::D),
                '0'...'9' => {
                    let mut number = (ch as i32) - ('0' as i32);
                    let mut it = self.iter.clone().peekable();
                    while let Some(&d) = it.peek() {
                        if !d.is_digit(10) {
                            break
                        }
                        number = (number * 10) + ((d as i32) - ('0' as i32));
                        
                        it.next();
                        self.iter.next();
                    }
                    return Some(Token::Number(number))
                },
                '(' => return Some(Token::OpenParren),
                ')' => return Some(Token::CloseParren),
                '+' => return Some(Token::AddOp),
                '-' => return Some(Token::SubOp),
                _ => { /* skip anything else */ }
            } 
        }
        None
    }
} 


pub fn scan(s: &String) -> Result<Vec<Token>, RollError> {
    let scaner = Scanner::new(s.chars());
    Ok(scaner.collect())
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
                assert!(result[i] == *item, "{:?}", result);
            }
        } else {
            assert!(false);
        }
    }
}
