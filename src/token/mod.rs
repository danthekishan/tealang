#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Illegal,
    Eof,

    Let,
    Ident,
    Define,
    Function,

    Assign,
    Comma,
    Semicolan,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LessThan,

    Int,
    Plus,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(tt: TokenType, l: String) -> Self {
        Token {
            token_type: tt,
            literal: l,
        }
    }

    pub fn lookup_ident(ident: String) -> Self {
        let tt = match ident.as_str() {
            "let" => TokenType::Let,
            "fn" => TokenType::Function,
            _ => TokenType::Ident,
        };

        Token {
            token_type: tt,
            literal: ident,
        }
    }
}
