use crate::{
    ast::identifier::Identifier,
    error::{Error, FileId, Result, diagnostic, error},
};
use codespan_reporting::diagnostic::Diagnostic;
use std::{ops::Range, str::CharIndices};
use symbol_table::{GlobalSymbol, static_symbol};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Source {
    file_id: FileId,
    range: Range<usize>,
}

impl Source {
    pub fn new(file_id: FileId, range: Range<usize>) -> Source {
        Source { file_id, range }
    }

    pub fn union(left: &Source, right: &Source) -> Source {
        Source {
            file_id: left.file_id,
            range: left.range.start..right.range.end,
        }
    }

    pub fn file_id(&self) -> FileId {
        self.file_id
    }

    pub fn range(&self) -> Range<usize> {
        self.range.clone()
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

impl From<Token> for TokenType {
    fn from(value: Token) -> Self {
        value.token_type
    }
}

impl From<Token> for Source {
    fn from(value: Token) -> Self {
        value.source
    }
}

impl<'a> From<&'a Token> for &'a Source {
    fn from(value: &'a Token) -> &'a Source {
        value.source()
    }
}

impl TryFrom<&Vec<Identifier>> for Source {
    type Error = crate::error::Error;

    fn try_from(value: &Vec<Identifier>) -> Result<Source> {
        if value.is_empty() {
            return error(Diagnostic::error().with_message("expected that type path is not empty"));
        }
        Ok(Source::union(
            value
                .first()
                .expect("expect first identifier exists")
                .source(),
            value
                .last()
                .expect("expect last identifier exists")
                .source(),
        ))
    }
}

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

impl TokenType {
    pub fn is_identifier(&self) -> bool {
        matches!(self, Self::Identifier(_))
    }
}

#[derive(Clone)]
pub struct Lexer<'a> {
    source: &'a str,
    chars: CharIndices<'a>,
    file_id: FileId,
    line: usize,
    start: usize,
    last: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str, file_id: FileId) -> Lexer<'a> {
        Lexer {
            source: source,
            chars: source.char_indices(),
            file_id,
            line: 1,
            start: 0,
            last: None,
        }
    }

    fn scan_token(&mut self) -> Result<Token> {
        let c = self.advance()?;
        let token_type = match c {
            '(' => TokenType::ParenLeft,
            ')' => TokenType::ParenRight,
            '{' => TokenType::BraceLeft,
            '}' => TokenType::BraceRight,
            '[' => TokenType::BracketLeft,
            ']' => TokenType::BracketRight,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            ':' => TokenType::Colon,
            ';' => TokenType::Semicolon,
            '+' => {
                if self.matchc('=') {
                    TokenType::SetAdd
                } else {
                    TokenType::Add
                }
            }
            '-' => {
                if self.matchc('=') {
                    TokenType::SetSub
                } else if self.matchc('>') {
                    TokenType::To
                } else {
                    TokenType::Sub
                }
            }
            '*' => {
                if self.matchc('=') {
                    TokenType::SetStar
                } else {
                    TokenType::Star
                }
            }
            '/' => {
                if self.matchc('/') {
                    while !self.is_at_end() && self.peek()? != '\n' {
                        self.advance()?;
                    }
                    return self.scan_token();
                } else if self.matchc('=') {
                    TokenType::SetDiv
                } else {
                    TokenType::Div
                }
            }
            '%' => TokenType::Mod,
            '&' => {
                if self.matchc('&') {
                    TokenType::And
                } else {
                    TokenType::Ref
                }
            }
            '|' => {
                if self.matchc('|') {
                    TokenType::Or
                } else {
                    TokenType::Vbar
                }
            }
            '=' => {
                if self.matchc('=') {
                    TokenType::Eq
                } else {
                    TokenType::Set
                }
            }
            '!' => {
                if self.matchc('=') {
                    TokenType::Neq
                } else {
                    TokenType::Not
                }
            }
            '>' => {
                if self.matchc('=') {
                    TokenType::Geq
                } else {
                    TokenType::Gre
                }
            }
            '<' => {
                if self.matchc('=') {
                    TokenType::Leq
                } else {
                    TokenType::Low
                }
            }
            ' ' | '\r' | '\t' => {
                self.start = self.chars.offset();
                return self.scan_token();
            }
            '\n' => {
                self.line += 1;
                self.start = self.chars.offset();
                return self.scan_token();
            }
            '"' => self.string()?,
            _ => {
                if Lexer::is_digit(c) {
                    self.number()?
                } else if Lexer::is_alpha(c) {
                    self.identifier()?
                } else {
                    return error(diagnostic(
                        format!("unexpected character '{}'", c),
                        &self.make_source(),
                    ));
                }
            }
        };
        Ok(Token::new(token_type, self.make_source()))
    }

    fn identifier(&mut self) -> Result<TokenType> {
        self.start = self.chars.offset() - 1;
        while !self.is_at_end() && Lexer::is_alpha_numeric(self.peek()?) {
            self.advance()?;
        }

        let text = self.view();
        Ok(match text {
            "import" => TokenType::Import,
            "struct" => TokenType::Struct,
            "io" => TokenType::Io,
            "in" => TokenType::In,
            "fn" => TokenType::Function,
            "const" => TokenType::Const,
            "extern" => TokenType::Extern,
            "let" => TokenType::Let,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "for" => TokenType::For,
            "while" => TokenType::While,
            "return" => TokenType::Return,
            "break" => TokenType::Break,
            "self" => TokenType::VSelf,
            "true" => TokenType::Bool(true),
            "false" => TokenType::Bool(false),
            _ => TokenType::Identifier(text.into()),
        })
    }

    fn number(&mut self) -> Result<TokenType> {
        self.start = self.chars.offset() - 1;
        let mut is_float = false;
        while !self.is_at_end() {
            let next = self.peek()?;
            if Lexer::is_digit(next) {
                self.advance()?;
            } else if next == '.' {
                if is_float {
                    return error(diagnostic(
                        "found a second dot in the float",
                        &self.make_source(),
                    ));
                }
                is_float = true;
                self.advance()?;
            } else {
                break;
            }
        }

        Ok(TokenType::Number(self.view().parse().unwrap()))
    }

    /// Parses a single string.
    fn string(&mut self) -> crate::error::Result<TokenType> {
        self.start = self.chars.offset();
        while !self.is_at_end() && self.peek()? != '"' {
            let c = self.advance()?;
            if c == '\n' {
                self.line += 1;
            }
        }

        let view = self.view().to_string();
        // The enclosing ".
        self.advance()?;

        Ok(TokenType::String(view))
    }

    fn matchc(&mut self, c: char) -> bool {
        match self.peek() {
            Ok(p) => p == c && self.advance().is_ok(),
            _ => false,
        }
    }

    fn peek(&mut self) -> Result<char> {
        if let Some(c) = self.last {
            return Ok(c);
        }
        let c = self.advance()?;
        self.last = Some(c);
        Ok(c)
    }

    fn is_alpha(c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    fn is_digit(c: char) -> bool {
        c.is_ascii_digit()
    }

    fn is_alpha_numeric(c: char) -> bool {
        Lexer::is_alpha(c) || Lexer::is_digit(c)
    }

    fn is_at_end(&self) -> bool {
        self.source.len() <= self.chars.offset()
    }

    fn advance(&mut self) -> Result<char> {
        if let Some(c) = self.last.take() {
            return Ok(c);
        }
        self.chars
            .next()
            .map(|v| v.1)
            .ok_or_else(|| Error::new(vec![Diagnostic::error().with_message("unexpected eof")]))
    }

    fn view(&self) -> &str {
        &self.source[self.start..self.chars.offset() - 1]
    }

    /// Public because it is used for error
    pub(crate) fn make_source(&self) -> Source {
        Source::new(self.file_id, self.start..self.chars.offset())
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_at_end() {
            None
        } else {
            Some(self.scan_token())
        }
    }
}
