use crate::{ast::identifier::Identifier, lexer::source::Source};
use symbol_table::{GlobalSymbol, static_symbol};

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum TokenType {
    // Brackets
    ParenLeft,    // (
    ParenRight,   // )
    BraceLeft,    // {
    BraceRight,   // }
    BracketLeft,  // [
    BracketRight, // ]
    Dot,
    Comma,     // ,
    Colon,     // :
    Semicolon, // ;
    To,        // ->

    // Operators
    Add,
    Sub,
    Div,
    Star,
    Set,
    SetAdd,
    SetSub,
    SetStar,
    SetDiv,
    Mod,
    Low,
    Gre,
    Geq,
    Leq,
    Eq,
    Neq,
    Not,
    And,
    Or,
    Vbar,
    Ref,

    // Keywords
    Import,
    Io,
    In,
    Function,
    Struct,
    Enum,
    If,
    Else,
    While,
    For,
    Match,
    Let,
    Const,
    Extern,
    Return,
    Break,
    VSelf,

    // Literal
    Bool(bool),
    Number(f64),
    String(String),
    Identifier(GlobalSymbol),

    // Misc
    Eof,
}

impl From<Token> for TokenType {
    fn from(value: Token) -> Self {
        value.token_type
    }
}

impl TokenType {
    pub fn is_identifier(&self) -> bool {
        matches!(self, Self::Identifier(_))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    token_type: TokenType,
    source: Source,
}

impl Token {
    pub fn new(token_type: TokenType, source: Source) -> Token {
        Token { token_type, source }
    }

    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn source(&self) -> &Source {
        &self.source
    }
}

impl From<Token> for Source {
    fn from(value: Token) -> Self {
        value.source
    }
}

impl From<Token> for Identifier {
    fn from(token: Token) -> Self {
        match token.token_type() {
            TokenType::Identifier(name) => Identifier::new(*name, token.source),
            TokenType::VSelf => Identifier::new(static_symbol!("self"), token.source),
            // TODO: Maybe replace with TryFrom?
            _ => panic!("token '{:?}' is not a identifier", token),
        }
    }
}

impl From<&Token> for Identifier {
    fn from(token: &Token) -> Self {
        if let TokenType::Identifier(name) = token.token_type() {
            Identifier::new(*name, token.source.clone())
        } else {
            // TODO: Maybe replace with TryFrom?
            panic!("can only be used on an identifier")
        }
    }
}
