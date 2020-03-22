use crate::error::{ErrorKind, MachinaError};

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    // symbols
    LParen,   // (
    RParen,   // )
    LBrace,   // {
    RBrace,   // }
    LBracket, // [
    RBracket, // ]
    Comma,    // ;
    Semi,     // ;
    Colon,    // :
    Equals,   // =

    // instructions
    Jump,
    JumpT,
    JumpF,
    Load,
    Store,
    Const,
    Input,
    Output,
    Return,
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
    Instruction,
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
        "jump"   => Some(TokenKind::Jump),
        "jumpt"  => Some(TokenKind::JumpT),
        "jumpf"  => Some(TokenKind::JumpF),
        "const"  => Some(TokenKind::Const),
        "load"   => Some(TokenKind::Load),
        "store"  => Some(TokenKind::Store),
        "input"  => Some(TokenKind::Input),
        "output" => Some(TokenKind::Output),
        "return" => Some(TokenKind::Return),
        "call"   => Some(TokenKind::Call),
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
                Some('$') => {
                    self.variable()
                }
                Some('a'..='z')
              | Some('A'..='Z')
              | Some('_') => {
                    self.identifier()
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
                    self.make_token(TokenKind::Semi)
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
            TokenKind::LParen   => write!(f, "("),
            TokenKind::RParen   => write!(f, ")"),
            TokenKind::LBrace   => write!(f, "{{"),
            TokenKind::RBrace   => write!(f, "}}"),
            TokenKind::LBracket => write!(f, "["),
            TokenKind::RBracket => write!(f, "]"),
            TokenKind::Comma    => write!(f, ";"),
            TokenKind::Semi     => write!(f, ";"),
            TokenKind::Colon    => write!(f, ":"),
            TokenKind::Equals   => write!(f, "="),
            TokenKind::Jump     => write!(f, "jump"),
            TokenKind::JumpT    => write!(f, "jumpt"),
            TokenKind::JumpF    => write!(f, "jumpf"),
            TokenKind::Load     => write!(f, "load"),
            TokenKind::Store    => write!(f, "store"),
            TokenKind::Const    => write!(f, "const"),
            TokenKind::Input    => write!(f, "input"),
            TokenKind::Output   => write!(f, "output"),
            TokenKind::Return   => write!(f, "return"),
            TokenKind::Call     => write!(f, "call"),
            TokenKind::Add      => write!(f, "add"),
            TokenKind::Sub      => write!(f, "sub"),
            TokenKind::Mul      => write!(f, "mul"),
            TokenKind::Div      => write!(f, "div"),
            TokenKind::Mod      => write!(f, "mod"),
            TokenKind::Eq       => write!(f, "eq"),
            TokenKind::Ne       => write!(f, "ne"),
            TokenKind::Lt       => write!(f, "lt"),
            TokenKind::Lte      => write!(f, "lte"),
            TokenKind::Gt       => write!(f, "gt"),
            TokenKind::Gte      => write!(f, "gte"),
            TokenKind::And      => write!(f, "and"),
            TokenKind::Or       => write!(f, "or"),
            TokenKind::Xor      => write!(f, "xor"),
            TokenKind::Not      => write!(f, "not"),
            TokenKind::Null     => write!(f, "null"),
            TokenKind::String   => write!(f, "string"),
            TokenKind::Integer  => write!(f, "integer"),
            TokenKind::Decimal  => write!(f, "decimal"),
            TokenKind::Variable => write!(f, "variable"),
            TokenKind::Label    => write!(f, "label"),
            TokenKind::Instruction => write!(f, "instruction"),
            TokenKind::EOF => write!(f, "end of file")
        }
    }
}