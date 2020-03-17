use crate::error::{ErrorKind, MachinaError};

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    // symbols
    LParen,      // (
    RParen,      // )
    LBrace,      // {
    RBrace,      // }
    LBracket,    // [
    RBracket,    // ]
    Comma,       // ;
    Semicolon,   // ;
    Colon,       // :
    Equals,      // =

    // instructions
    Define,
    End,
    Jmp,
    JmpT,
    JmpF,
    If,
    Switch,
    Out,
    Ret,

    // expressions
    In,
    Call,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Lt,
    Lte,
    Gt,
    Gte,
    And,
    Or,
    Xor,
    Not,
    Null,

    // values
    String,
    Integer,
    Decimal,
    Variable,
    Label,

    // other
    EOF
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: Option<String>,
    pub line: u32
}


fn get_keyword(key: &str) -> Option<TokenKind> {
    match key {
        "define" => Some(TokenKind::Define),
        "end"    => Some(TokenKind::End),
        "if"     => Some(TokenKind::If),
        "jmp"    => Some(TokenKind::Jmp),
        "jmpt"   => Some(TokenKind::JmpT),
        "jmpf"   => Some(TokenKind::JmpF),
        "out"    => Some(TokenKind::Out),
        "in"     => Some(TokenKind::In),
        "ret"    => Some(TokenKind::Ret),
        "call"   => Some(TokenKind::Call),
        "switch" => Some(TokenKind::Switch),
        "add"    => Some(TokenKind::Add),
        "sub"    => Some(TokenKind::Sub),
        "mul"    => Some(TokenKind::Mul),
        "div"    => Some(TokenKind::Div),
        "mod"    => Some(TokenKind::Mod),
        "eq"     => Some(TokenKind::Eq),
        "ne"     => Some(TokenKind::Ne),
        "lt"     => Some(TokenKind::Lt),
        "lte"    => Some(TokenKind::Lte),
        "gt"     => Some(TokenKind::Gt),
        "gte"    => Some(TokenKind::Gte),
        "and"    => Some(TokenKind::And),
        "or"     => Some(TokenKind::Or),
        "xor"    => Some(TokenKind::Xor),
        "not"    => Some(TokenKind::Not),
        "null"   => Some(TokenKind::Null),
        _ => None
    }
}

type LexerResult = Result<Token, MachinaError>;

#[derive(Debug, Clone)]
pub struct Lexer<T: Iterator<Item = char>> {
    source: T,
    curr: Option<char>,
    peek: Option<char>,
    line: u32,
}

impl<T> Lexer<T>
where
    T: Iterator<Item = char>,
{
    pub fn new(source: T) -> Self {
        let mut lexer = Lexer {
            source,
            curr: None,
            peek: None,
            line: 1,
        };
        lexer.next_char(); // curr
        lexer.next_char(); // peek
        lexer.line = 1;
        lexer
    }

    fn next_token(&mut self) -> Option<LexerResult> {
        loop {
            let token = match self.curr {
                Some('a'..='z')
              | Some('A'..='Z')
              | Some('_') => {
                    self.identifier()
                }
                Some('$') => {
                    self.variable()
                }
                Some('0'..='9') => {
                    self.number(false)
                },
                  Some('-')
                | Some('+') if self.is_number(self.peek) => {
                    self.number(true)
                }
                Some(' ') => {
                    self.next_char();
                    continue;
                }
                Some('\r') => {
                    self.next_char();
                    continue;
                }
                Some('\n') => {
                    self.new_line();
                    self.next_char();
                    continue;
                }
                Some('#') => {
                    self.comment();
                    continue;
                }
                Some('"') => {
                    self.string()
                }
                Some('=') => {
                    self.make_token(TokenKind::Equals)
                }
                Some(':') => {
                    self.make_token(TokenKind::Colon)
                }
                Some(',') => {
                    self.make_token(TokenKind::Comma)
                }
                Some(';') => {
                    self.make_token(TokenKind::Semicolon)
                }
                Some('(') => {
                    self.make_token(TokenKind::LParen)
                }
                Some(')') => {
                    self.make_token(TokenKind::RParen)
                }
                Some('{') => {
                    self.make_token(TokenKind::LBrace)
                }
                Some('}') => {
                    self.make_token(TokenKind::RBrace)
                }
                Some('[') => {
                    self.make_token(TokenKind::LBracket)
                }
                Some(']') => {
                    self.make_token(TokenKind::RBracket)
                }
                Some(chr) => {
                    self.next_char();
                    return Some(Err(MachinaError { kind: ErrorKind::InvalidCharacter(chr), line: self.line }))
                }
                None => {
                    return None
                }
            };
            return Some(token)
        }
    }

    fn make_token(&mut self, kind: TokenKind) -> LexerResult {
        self.next_char();
        Ok(Token { kind, value: None, line: self.line })
    }

    fn next_char(&mut self) -> Option<char> {
        let curr = self.curr;
        self.curr = self.peek;
        self.peek = self.source.next();
        curr
    }

    fn new_line(&mut self) {
        self.line += 1;
    }

    fn is_alpha(&self, chr: Option<char>) -> bool {
        match chr {
            Some('a'..='z')
          | Some('A'..='Z')
          | Some('0'..='9')
          | Some('_') => true,
            _ => false,
        }
    }

    fn is_number(&self, chr: Option<char>) -> bool {
        match chr {
            Some('0'..='9') => true,
            _ => false,
        }
    }

    fn identifier(&mut self) -> LexerResult {
        let raw = self.word(String::new());

        match get_keyword(&raw) {
            Some(kind) => {
                Ok(Token {
                    value: None,
                    kind,
                    line: self.line,
                })
            }
            None => {
                Ok(Token {
                    kind: TokenKind::Label,
                    value: Some(raw),
                    line: self.line,
                })
            }
        }
    }

    fn variable(&mut self) -> LexerResult {
        self.next_char();
        let raw = self.word("$".into());

        Ok(Token {
            kind: TokenKind::Variable,
            value: Some(raw),
            line: self.line,
        })
    }

    fn word(&mut self, mut from: String) -> String {
        from.push(self.next_char().unwrap());
        while self.is_alpha(self.curr) {
            from.push(self.next_char().unwrap());
        }
        from
    }

    fn number(&mut self, prefix: bool) -> LexerResult {
        let mut raw = String::new();

        if prefix {
            raw.push(self.next_char().unwrap());
        }

        // integer
        while self.is_number(self.curr) {
            raw.push(self.next_char().unwrap());
        }

        // decimal
        if self.curr == Some('.') && self.is_number(self.peek) {
            raw.push(self.next_char().unwrap());
            while self.is_number(self.curr) {
                raw.push(self.next_char().unwrap());
            }
            Ok(Token {
                kind: TokenKind::Decimal,
                value: Some(raw),
                line: self.line,
            })
        } else {
            Ok(Token {
                kind: TokenKind::Integer,
                value: Some(raw),
                line: self.line,
            })
        }
    }

    fn string(&mut self) -> LexerResult {
        self.next_char().unwrap();

        let mut raw = String::new();
        loop {
            match self.next_char() {
                Some('\\') => {
                    match self.next_char() {
                        Some('\n') => {
                            self.new_line();
                        }
                        Some('\\') => raw.push('\\'),
                        Some('\'') => raw.push('\''),
                        Some('\"') => raw.push('\"'),
                        Some('n')  => raw.push('\n'),
                        Some('r')  => raw.push('\r'),
                        Some('t')  => raw.push('\t'),
                        Some('a')  => raw.push('\x07'),
                        Some('b')  => raw.push('\x08'),
                        Some('f')  => raw.push('\x0c'),
                        Some('v')  => raw.push('\x0b'),
                        Some(chr)  => {
                            raw.push('\\');
                            raw.push(chr);
                        }
                        None => {
                            return Err(MachinaError {
                                kind: ErrorKind::InvalidEscapeCharacter,
                                line: self.line
                            });
                        }
                    }
                }
                Some(chr) => {
                    if chr == '"' {
                        break;
                    } else {
                        raw.push(chr);
                    }
                }
                None => {
                    return Err(MachinaError {
                        kind: ErrorKind::UnterminatedString,
                        line: self.line
                    });
                }
            }
        }
        Ok(Token {
            kind: TokenKind::String,
            value: Some(raw),
            line: self.line,
        })
    }

    fn comment(&mut self) {
        loop {
            match self.curr {
                Some('\n') => {
                    return;
                }
                Some(_) => {}
                None => return,
            }
            self.next_char();
        }
    }
}

impl<T> Iterator for Lexer<T>
where
    T: Iterator<Item = char>,
{
    type Item = LexerResult;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenKind::LParen    => write!(f, "("),
            TokenKind::RParen    => write!(f, ")"),
            TokenKind::LBrace    => write!(f, "{{"),
            TokenKind::RBrace    => write!(f, "}}"),
            TokenKind::LBracket  => write!(f, "["),
            TokenKind::RBracket  => write!(f, "]"),
            TokenKind::Comma     => write!(f, ","),
            TokenKind::Semicolon => write!(f, ";"),
            TokenKind::Colon     => write!(f, ":"),
            TokenKind::Equals    => write!(f, "="),
            TokenKind::Define    => write!(f, "define"),
            TokenKind::End       => write!(f, "end"),
            TokenKind::If        => write!(f, "if"),
            TokenKind::Jmp       => write!(f, "jmp"),
            TokenKind::JmpT      => write!(f, "jmpt"),
            TokenKind::JmpF      => write!(f, "jmpf"),
            TokenKind::Out       => write!(f, "out"),
            TokenKind::In        => write!(f, "in"),
            TokenKind::Ret       => write!(f, "ret"),
            TokenKind::Call      => write!(f, "call"),
            TokenKind::Switch    => write!(f, "switch"),
            TokenKind::Add       => write!(f, "add"),
            TokenKind::Sub       => write!(f, "sub"),
            TokenKind::Mul       => write!(f, "mul"),
            TokenKind::Div       => write!(f, "div"),
            TokenKind::Mod       => write!(f, "mod"),
            TokenKind::Eq        => write!(f, "eq"),
            TokenKind::Ne        => write!(f, "ne"),
            TokenKind::Lt        => write!(f, "lt"),
            TokenKind::Lte       => write!(f, "lte"),
            TokenKind::Gt        => write!(f, "gt"),
            TokenKind::Gte       => write!(f, "gte"),
            TokenKind::And       => write!(f, "and"),
            TokenKind::Or        => write!(f, "or"),
            TokenKind::Xor       => write!(f, "xor"),
            TokenKind::Not       => write!(f, "not"),
            TokenKind::Null      => write!(f, "null"),
            TokenKind::String    => write!(f, "string"),
            TokenKind::Integer   => write!(f, "integer"),
            TokenKind::Decimal   => write!(f, "decimal"),
            TokenKind::Variable  => write!(f, "variable"),
            TokenKind::Label     => write!(f, "label"),
            TokenKind::EOF       => write!(f, "end of file"),
        }
    }
}