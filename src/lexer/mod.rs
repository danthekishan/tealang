use std::{iter::Peekable, slice::Iter};

use crate::token::{Token, TokenType};

// peekable iterator holds a reference to &str
// it requires lifetime annotation
// to make sure lexer knows the &str lifetime
pub struct Lexer<'a> {
    iter: Peekable<Iter<'a, u8>>,
}

// like struct impl block also should have it
impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            iter: input.as_bytes().iter().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        while self.iter.next_if(|&x| x.is_ascii_whitespace()).is_some() {}

        let Some(&next_char) = self.iter.next() else {
            return Token::new(TokenType::Eof, "".to_string());
        };

        // handling continous string, including '_'
        if next_char.is_ascii_alphabetic() || next_char == b'_' {
            let mut buffer = String::new();
            buffer.push(next_char as char);

            while let Some(&ch) = self
                .iter
                .next_if(|&x| x.is_ascii_alphanumeric() || *x == b'_')
            {
                buffer.push(ch as char);
            }
            return Token::lookup_ident(buffer);
        }

        // handling continous number - mainly integer
        if next_char.is_ascii_digit() {
            let mut buffer = String::new();
            buffer.push(next_char as char);

            while let Some(&ch) = self.iter.next_if(|&x| x.is_ascii_digit()) {
                buffer.push(ch as char);
            }

            return Token::new(TokenType::Int, buffer);
        }

        match next_char {
            b'=' => {
                if let Some(&&peek) = self.iter.peek()
                    && peek == b'='
                {
                    Token::new(TokenType::EQ, "=".to_string())
                } else {
                    Token::new(TokenType::Assign, "=".to_string())
                }
            }
            b',' => Token::new(TokenType::Comma, ",".to_string()),
            b';' => Token::new(TokenType::Semicolan, ";".to_string()),
            b'(' => Token::new(TokenType::LParen, "(".to_string()),
            b')' => Token::new(TokenType::RParen, ")".to_string()),
            b'{' => Token::new(TokenType::LBrace, "{".to_string()),
            b'}' => Token::new(TokenType::RBrace, "}".to_string()),
            b'!' => {
                if let Some(&&peek) = self.iter.peek()
                    && peek == b'='
                {
                    Token::new(TokenType::NotEQ, "=".to_string())
                } else {
                    Token::new(TokenType::Bang, "!".to_string())
                }
            }
            b'+' => Token::new(TokenType::Plus, "+".to_string()),
            b'-' => Token::new(TokenType::Minus, "-".to_string()),
            b'*' => Token::new(TokenType::Asterisk, "*".to_string()),
            b'/' => Token::new(TokenType::Slash, "/".to_string()),
            b'<' => {
                if let Some(&&peek) = self.iter.peek()
                    && peek == b'-'
                {
                    self.iter.next();
                    Token::new(TokenType::Define, "<-".to_string())
                } else {
                    Token::new(TokenType::LT, "<".to_string())
                }
            }
            b'>' => Token::new(TokenType::GT, ">".to_string()),
            _ => Token::new(TokenType::Illegal, "".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = r#"
let five = 5;
let ten = 10;

add <- fn(x, y) {
    x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;

Calculator <- struct {
    name: str
}

Calculator <- new(name) {
    return Calculator {name: name};
}

Calculator <- extend(self) {
    add <- fn(a, b) {return a + b}
    sub <- fn(a, b) {return a - b}
}
"#;

        let tests = vec![
            // let five = 5;
            TokenType::Let,
            TokenType::Ident,
            TokenType::Assign,
            TokenType::Int,
            TokenType::Semicolan,
            // let ten = 10;
            TokenType::Let,
            TokenType::Ident,
            TokenType::Assign,
            TokenType::Int,
            TokenType::Semicolan,
            // add <- fn(x, y) {
            TokenType::Ident,
            TokenType::Define,
            TokenType::Function,
            TokenType::LParen,
            TokenType::Ident,
            TokenType::Comma,
            TokenType::Ident,
            TokenType::RParen,
            TokenType::LBrace,
            // x + y;
            TokenType::Ident,
            TokenType::Plus,
            TokenType::Ident,
            TokenType::Semicolan,
            // };
            TokenType::RBrace,
            TokenType::Semicolan,
            // let result = add(five, ten);
            TokenType::Let,
            TokenType::Ident,
            TokenType::Assign,
            TokenType::Ident,
            TokenType::LParen,
            TokenType::Ident,
            TokenType::Comma,
            TokenType::Ident,
            TokenType::RParen,
            TokenType::Semicolan,
            // !-/*5;
            TokenType::Bang,
            TokenType::Minus,
            TokenType::Slash,
            TokenType::Asterisk,
            TokenType::Int,
            TokenType::Semicolan,
            // 5 < 10 > 5;
            TokenType::Int,
            TokenType::LT,
            TokenType::Int,
            TokenType::GT,
            TokenType::Int,
            TokenType::Semicolan,
            // if (5 < 10) {
            TokenType::If,
            TokenType::LParen,
            TokenType::Int,
            TokenType::LT,
            TokenType::Int,
            TokenType::RParen,
            TokenType::LBrace,
            // return true;
            TokenType::Return,
            TokenType::True,
            TokenType::Semicolan,
            // } else {
            TokenType::RBrace,
            TokenType::Else,
            TokenType::LBrace,
            // return false;
            TokenType::Return,
            TokenType::False,
            TokenType::Semicolan,
            // }
            TokenType::RBrace,
        ];

        let mut lexer = Lexer::new(input);

        for expected in tests {
            let tok = lexer.next_token();
            println!("{}", tok.literal);
            assert_eq!(tok.token_type, expected);
        }
    }
}
