use crate::{
    ast::identifier::Identifier,
    error::{FileId, Result, diagnostic, error},
};
use codespan_reporting::diagnostic::Diagnostic;
use std::{collections::VecDeque, iter::Peekable, ops::Range, str::Chars};

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
            TokenType::Identifier(name) => Identifier::new(name.to_string(), token.source),
            TokenType::VSelf => Identifier::new("self".to_string(), token.source),
            _ => panic!("token '{:?}' is not a identifier", token),
        }
    }
}

impl From<&Token> for Identifier {
    fn from(token: &Token) -> Self {
        if let TokenType::Identifier(name) = token.token_type() {
            Identifier::new(name.to_string(), token.source.clone())
        } else {
            panic!()
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
    Identifier(String),

    // Misc
    Eof,
}

impl TokenType {
    pub fn is_identifer(&self) -> bool {
        matches!(self, Self::Identifier(_))
    }
}

#[derive(Clone)]
pub struct Lexer<'a> {
    source: Peekable<Chars<'a>>,
    tokens: VecDeque<Token>,
    file_id: FileId,
    line: usize,
    start: usize,
    offset: usize,
}

impl Lexer<'_> {
    pub fn new(source: Chars<'_>, file_id: FileId) -> Lexer<'_> {
        Lexer {
            source: source.peekable(),
            file_id,
            tokens: VecDeque::new(),
            line: 1,
            start: 0,
            offset: 0,
        }
    }

    pub fn scan(mut self) -> Result<VecDeque<Token>> {
        let mut error = Vec::new();
        while !self.is_at_end() {
            if let Err(e) = self.scan_token() {
                error.push(e);
            }
        }
        if !error.is_empty() {
            Err(error.into_iter().reduce(|a, b| a.concat(b)).unwrap())
        } else {
            Ok(self.tokens)
        }
    }

    fn scan_token(&mut self) -> Result<()> {
        let token_type = if let Some(c) = self.advance() {
            match c {
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
                        while !self.is_at_end() {
                            if let Some(c) = self.peek() {
                                if *c == '\n' {
                                    break;
                                }
                                self.advance();
                            } else {
                                // Reached EOF
                                break;
                            }
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
                    self.start = self.offset;
                    return self.scan_token();
                }
                '\n' => {
                    self.line += 1;
                    self.start = self.offset;
                    return self.scan_token();
                }
                '"' => self.string()?,
                _ => {
                    if Lexer::is_digit(c) {
                        self.number(c)?
                    } else if Lexer::is_alpha(c) {
                        self.identifier(c)?
                    } else {
                        return error(diagnostic(
                            format!("unexpected character '{}'", c),
                            &self.make_source(),
                        ));
                    }
                }
            }
        } else {
            TokenType::Eof
        };
        self.add_token(token_type);
        Ok(())
    }

    fn identifier(&mut self, c: char) -> Result<TokenType> {
        let mut text = String::from(c);
        while !self.is_at_end() {
            if let Some(c) = self.peek() {
                let c = *c;
                if !Lexer::is_alpha_numeric(c) {
                    break;
                }
                self.advance();
                text.push(c)
            } else {
                return error(diagnostic("unexpected EOF", &self.make_source()));
            }
        }

        Ok(match text.as_str() {
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
            _ => TokenType::Identifier(text),
        })
    }

    fn number(&mut self, c: char) -> Result<TokenType> {
        let mut text = String::from(c);
        let mut is_float = false;
        while !self.is_at_end() {
            let next = if let Some(next) = self.peek() {
                *next
            } else {
                return error(diagnostic("unexpected EOF", &self.make_source()));
            };
            if Lexer::is_digit(next) {
                if let Some(c) = self.advance() {
                    text.push(c)
                } else {
                    return error(diagnostic(
                        "unexpected EOF, expected a digit",
                        &self.make_source(),
                    ));
                }
            } else if next == '.' {
                if is_float {
                    return error(diagnostic(
                        "found a second dot in the float",
                        &self.make_source(),
                    ));
                }
                is_float = true;
                if let Some(c) = self.advance() {
                    text.push(c)
                } else {
                    return error(diagnostic(
                        "found EOF, expected a dot ('.')",
                        &self.make_source(),
                    ));
                }
            } else {
                break;
            }
        }

        Ok(TokenType::Number(
            text.parse()
                .expect("lexer should have only parsed a valid number"),
        ))
    }

    fn string(&mut self) -> crate::error::Result<TokenType> {
        let mut text = String::new();
        // FIX:
        while *self
            .peek()
                .unwrap()
            // .ok_or(lexer_expected("got EOF in unterminated String", self))?
            != '"'
        {
            if let Some(c) = self.advance() {
                if c == '\n' {
                    self.line += 1;
                }
                text.push(c);
            }
        }

        // The enclosing ".
        self.advance();

        Ok(TokenType::String(text))
    }

    fn matchc(&mut self, c: char) -> bool {
        match self.peek() {
            Some(p) => {
                if p != &c {
                    return false;
                }
                self.advance();
                true
            }
            _ => false,
        }
    }

    fn peek(&mut self) -> Option<&char> {
        self.source.peek()
    }

    fn is_alpha(c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    fn is_digit(c: char) -> bool {
        c.is_ascii_digit()
    }

    fn is_alpha_numeric(c: char) -> bool {
        Lexer::is_alpha(c) || Lexer::is_digit(c)
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().is_none()
    }

    fn advance(&mut self) -> Option<char> {
        self.offset += 1;
        self.source.next()
    }

    /// Public because it is used for error
    pub(crate) fn make_source(&self) -> Source {
        Source::new(self.file_id, self.start..self.offset)
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens
            .push_back(Token::new(token_type, self.make_source()))
    }
}
