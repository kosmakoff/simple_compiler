#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    LParen,
    RParen,
    Plus,
    Minus,
    Multiply,
    Divide,
    Identifier(String),
    Integer(i32),
    StringLiteral(String),
}

struct Interpreter {
    tokens: Vec<Token>,
    pos: usize
}

impl Interpreter {
    fn new(tokens: Vec<Token>) -> Self {
        Interpreter {
            tokens,
            pos: 0
        }
    }

    fn advance(&mut self) {
        self.pos = self.pos + 1;
    }

    fn current(&self) -> Option<Token> {
        if self.pos < self.tokens.len() {
            Some(self.tokens[self.pos].clone())
        } else {
            None
        }
    }

    fn eat_plus(&mut self) -> Result<(), String> {
        if let Some(Plus) = self.current() {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected PLUS, found {:?}", self.current()))
        }
    }

    fn eat_minus(&mut self) -> Result<(), String> {
        if let Some(Minus) = self.current() {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected MINUS, found {:?}", self.current()))
        }
    }

    fn eat_integer(&mut self) -> Result<i32, String> {
        if let Some(Integer(n)) = self.current() {
            self.advance();
            Ok(n)
        } else {
            Err(format!("Expected INTEGER, found {:?}", self.current()))
        }
    }

    fn expr(&mut self) -> Result<i32, String> {
        let mut result: i32 = 0;
        result = result + self.eat_integer()?;

        let current = self.current();

        match current {
            Some(Plus) => self.eat_plus()?,
            Some(Minus) => self.eat_minus()?,
            token @ _ => return Err(format!("Expected PLUS | MINUS, encountered: {:?}", token))
        }
        result = result + self.eat_integer()?;

        Ok(result)
    }
}

use Token::*;

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut vec = Vec::new();

    let mut iterator = input.chars().enumerate().peekable();

    while let Some((_, c)) = iterator.next() {
        let token = match c {
            '(' => LParen,
            ')' => RParen,
            '+' => Plus,
            '-' => Minus,
            '*' => Multiply,
            '/' => Divide,
            ' ' => continue,
            '0'...'9' => {
                let mut current = String::new();
                current.push(c);

                while let Some(&(_, n)) = iterator.peek() {
                    match n {
                        '0'...'9' => {
                            iterator.next();
                            current.push(n);
                        }
                        _ => break,
                    }
                }

                if let Ok(number) = current.parse::<i32>() {
                    Integer(number)
                } else {
                    return Err(String::from("Failed to parse number"));
                }
            }
            'a'...'z' => {
                let mut current = String::new();
                current.push(c);

                while let Some(&(_, n)) = iterator.peek() {
                    match n {
                        'a'...'z' | '0'...'9' => {
                            iterator.next();
                            current.push(n);
                        }
                        _ => break,
                    }
                }

                Identifier(current)
            }
            '\'' => {
                let mut current = String::new();

                while let Some((_, c)) = iterator.next() {
                    match c {
                        '\'' => break,
                        other => {
                            current.push(other);
                        }
                    }
                }

                StringLiteral(current)
            }
            _ => continue,
        };

        vec.push(token);
    }

    Ok(vec)
}

pub fn interpret(tokens: Vec<Token>) -> Result<i32, String> {
    let mut interpreter = Interpreter::new(tokens);

    interpreter.expr()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tokenizer_generates_correct_tokens() {
        let input = "(add 2 (subtract 44b ('hello world') 2))";
        let tokens = tokenize(&input).unwrap();

        assert_eq!(
            vec![
                LParen,
                Identifier(String::from("add")),
                Integer(2),
                LParen,
                Identifier(String::from("subtract")),
                Integer(44),
                Identifier(String::from("b")),
                LParen,
                StringLiteral(String::from("hello world")),
                RParen,
                Integer(2),
                RParen,
                RParen
            ],
            tokens
        );
    }

}
