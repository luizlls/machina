use crate::error::{ErrorKind, MachinaError};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    // symbols
    LParen,      // (
    RParen,      // )
    LBrace,      // {
    RBrace,      // }
    LBracket,    // [
    RBracket,    // ]
    Semicolon,   // ;
    Colon,       // :
    Equals,      // =

    // operations
    Proc,
    End,
    Case,
    Exec,
    If,
    Jmp,
    JmpT,
    JmpF,
    Call,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
    And,
    Or,
    Xor,
    Not,
    In,
    Out,


    // values
    String,
    Integer,
    Decimal,
    Variable,
    Identifier,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: Option<String>,
    pub line: u32
}


fn get_keyword(key: &str) -> Option<TokenKind> {
    match key {
        "proc" => Some(TokenKind::Proc),
        "end"  => Some(TokenKind::End),
        "case" => Some(TokenKind::Case),
        "exec" => Some(TokenKind::Exec),
        "if"   => Some(TokenKind::If),
        "jmp"  => Some(TokenKind::Jmp),
        "jmpt" => Some(TokenKind::JmpT),
        "jmpf" => Some(TokenKind::JmpF),
        "call" => Some(TokenKind::Call),
        "add"  => Some(TokenKind::Add),
        "sub"  => Some(TokenKind::Sub),
        "mul"  => Some(TokenKind::Mul),
        "div"  => Some(TokenKind::Div),
        "mod"  => Some(TokenKind::Mod),
        "eq"   => Some(TokenKind::Eq),
        "neq"  => Some(TokenKind::Neq),
        "lt"   => Some(TokenKind::Lt),
        "lte"  => Some(TokenKind::Lte),
        "gt"   => Some(TokenKind::Gt),
        "gte"  => Some(TokenKind::Gte),
        "and"  => Some(TokenKind::And),
        "or"   => Some(TokenKind::Or),
        "xor"  => Some(TokenKind::Xor),
        "not"  => Some(TokenKind::Not),
        "in"   => Some(TokenKind::In),
        "out"  => Some(TokenKind::Out),
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
                Some(_) => {
                    return Some(Err((ErrorKind::InvalidCharacter, self.line)))
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
                    kind,
                    value: Some(raw),
                    line: self.line,
                })
            }
            None => {
                Ok(Token {
                    kind: TokenKind::Identifier,
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
                            return Err((ErrorKind::InvalidEscapeCharacter, self.line));
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
                    return Err((ErrorKind::UnterminatedString, self.line));
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
