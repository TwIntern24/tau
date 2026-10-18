use crate::{
    ast::identifier::Identifier,
    error::{Diagnostic, Error, Result, lexer_expected},
};
use std::{
    fmt::{Display, Formatter},
    fs, io,
    path::PathBuf,
    rc::Rc,
    str::CharIndices,
};
use symbol_table::{GlobalSymbol, static_symbol};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Source {
    file: Rc<PathBuf>,
    line: usize,
    start: usize,
    end: usize,
}

#[cfg(test)]
impl Default for Source {
    fn default() -> Self {
        Source {
            file: Rc::new(PathBuf::new()),
            line: 0,
            start: 0,
            end: 0,
        }
    }
}

impl Source {
    pub fn new(file: Rc<PathBuf>, line: usize, start: usize, end: usize) -> Source {
        Source {
            file,
            line,
            start,
            end,
        }
    }

    pub fn union(left: &Source, right: &Source) -> Source {
        Source {
            file: left.file.clone(),
            line: left.line,
            start: left.start,
            end: right.end,
        }
    }

    pub fn content(&self) -> io::Result<String> {
        let content = fs::read_to_string(self.file.to_path_buf())?;

        return Ok(content[self.start..self.end].to_string());
    }
}

impl Display for Source {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(formatter, "{}:{}", self.file.to_string_lossy(), self.line)
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

    pub fn source(&self) -> Source {
        self.source.clone()
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
    pub fn is_identifer(&self) -> bool {
        matches!(self, Self::Identifier(_))
    }
}

#[derive(Clone)]
pub struct Lexer<'a> {
    source: &'a str,
    chars: CharIndices<'a>,
    file: Rc<PathBuf>,
    line: usize,
    start: usize,
    last: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str, file: Rc<PathBuf>) -> Lexer<'a> {
        Lexer {
            source: source,
            chars: source.char_indices(),
            file,
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
                    Err(Error::new(vec![Diagnostic::new(
                        format!("unexpected character '{}'", c),
                        self.make_source(),
                    )]))?
                }
            }
        };
        //} else {
        //    TokenType::Eof
        //};
        //self.add_token(token_type);
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
                    return Err(lexer_expected("found a second dot in the float", self));
                }
                is_float = true;
                self.advance()?;
            } else {
                break;
            }
        }

        Ok(TokenType::Number(dbg!(self.view()).parse().unwrap()))
    }

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

        Ok(TokenType::String(dbg!(view)))
    }

    fn matchc(&mut self, c: char) -> bool {
        match self.peek() {
            Ok(p) => {
                if p != c || self.advance().is_err() {
                    return false;
                }
                true
            }
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
            .ok_or_else(|| lexer_expected("unexpected eof", self))
    }

    fn view(&self) -> &str {
        &self.source[self.start..self.chars.offset() - 1]
    }

    /// Public because it is used for error
    pub(crate) fn make_source(&self) -> Source {
        Source::new(
            self.file.clone(),
            self.line,
            self.start,
            self.chars.offset(),
        )
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
