use crate::{error::{MachinaError, MachinaErrorKind}};

use std::{fmt, str::Chars};

fn get_instruction(key: &str) -> Option<Token> {
    match key {
        "call" => Some(Token::Call),
        "ret"  => Some(Token::Ret),
        "move" => Some(Token::Move),
        "jmp"  => Some(Token::Jmp),
        "jt"   => Some(Token::Jt),
        "jf"   => Some(Token::Jf),
        "jlt"  => Some(Token::JLt),
        "jle"  => Some(Token::JLe),
        "jgt"  => Some(Token::JGt),
        "jge"  => Some(Token::JGe),
        "jeq"  => Some(Token::JEq),
        "jne"  => Some(Token::JNe),
        "lt"   => Some(Token::Lt),
        "le"   => Some(Token::Le),
        "gt"   => Some(Token::Gt),
        "ge"   => Some(Token::Ge),
        "eq"   => Some(Token::Eq),
        "ne"   => Some(Token::Ne),
        "add"  => Some(Token::Add),
        "sub"  => Some(Token::Sub),
        "mul"  => Some(Token::Mul),
        "div"  => Some(Token::Div),
        "mod"  => Some(Token::Mod),
        "not"  => Some(Token::Not),
        "and"  => Some(Token::And),
        "or"   => Some(Token::Or),
        "xor"  => Some(Token::Xor),
        "shl"  => Some(Token::Shl),
        "shr"  => Some(Token::Shr),
        "write"  => Some(Token::Write),
        _ => None,
    }
}

type LexerResult = Result<Token, MachinaError>;

#[derive(Debug, Clone)]
pub struct Lexer<'s> {
    source: &'s str,
    chars: Chars<'s>,
    curr: Option<char>,
    peek: Option<char>,
    line: usize,
    value: Option<String>,
}

impl<'s> Lexer<'s> {
    pub fn new(source: &'s str) -> Lexer {
        let mut lexer = Lexer {
            source,
            chars: source.chars(),
            curr: None,
            peek: None,
            line: 0,
            value: None,
        };
        lexer.initialize();
        lexer
    }

    fn initialize(&mut self) {
        self.next_char();
        self.next_char();

        while self.curr == Some('\n') {
            self.line += 1;
            self.next_char();
        }
    }

    fn next_token(&mut self) -> LexerResult {
        loop {
            let token = match self.curr {
                Some(' ')
              | Some('\t')
              | Some('\r') => {
                    self.space();
                    continue;
                }
                Some('.') => {
                    self.identifier(Token::Label)
                }
                Some('%') => {
                    self.identifier(Token::Register)
                }
                Some('@') => {
                    self.identifier(Token::Function)
                }
                Some('a'..='z')
              | Some('A'..='Z') => {
                    self.instruction()
                }
                Some('0'..='9') => {
                    self.number(false)
                }
                Some('-') | Some('+') if self.is_number(self.peek) => {
                    self.number(true)
                }
                Some('"') => {
                    self.string()
                }
                Some('\n') => {
                    self.line += 1;
                    self.single(Token::EOL)
                }
                Some(',') => self.single(Token::Comma),
                Some('(') => self.single(Token::LParen),
                Some(')') => self.single(Token::RParen),
                Some('{') => self.single(Token::LBrace),
                Some('}') => self.single(Token::RBrace),
                Some('[') => self.single(Token::LBracket),
                Some(']') => self.single(Token::RBracket),
                Some(';') => {
                    self.comment();
                    continue;
                },
                Some(invalid) => {
                    Err(
                        MachinaError {
                            kind: MachinaErrorKind::InvalidCharacter(invalid), line: self.line
                        }
                    )
                }
                None => Ok(Token::EOF)
            };

            return token;
        }
    }

    fn single(&mut self, token: Token) -> LexerResult {
        self.next_char();
        Ok(token)
    }

    fn next_char(&mut self) -> Option<char> {
        let curr = self.curr;
        self.curr = self.peek;
        self.peek = self.chars.next();
        curr
    }

    fn is_alpha(&self, chr: Option<char>) -> bool {
        matches!(chr, Some('a'..='z') | Some('A'..='Z') | Some('0'..='9') | Some('_'))
    }

    fn is_number(&self, chr: Option<char>) -> bool {
        matches!(chr, Some('0'..='9'))
    }

    fn instruction(&mut self) -> LexerResult {
        let mut value = String::new();

        while self.is_alpha(self.curr) {
            value.push(self.next_char().unwrap());
        }

        if let Some(instruction) = get_instruction(&value[..].to_lowercase()) {
            Ok(instruction)
        } else {
            Err(
                MachinaError {
                    kind: MachinaErrorKind::InvalidInstruction(value.into()), line: self.line
                }
            )
        }
    }

    fn identifier(&mut self, kind: Token) -> LexerResult {
        let mut value = String::new();

        self.next_char(); // marker (#, @, .)

        while self.is_alpha(self.curr) {
            value.push(self.next_char().unwrap());
        }

        self.value = Some(value.into());

        Ok(kind)
    }

    fn number(&mut self, prefix: bool) -> LexerResult {
        let mut value = String::new();

        if prefix {
            value.push(self.next_char().unwrap());
        }

        while self.is_number(self.curr) {
            value.push(self.next_char().unwrap());
        }

        if self.curr == Some('.') && self.is_number(self.peek) {
            value.push(self.next_char().unwrap());

            while self.is_number(self.curr) {
                value.push(self.next_char().unwrap());
            }
        }

        self.value = Some(value.into());

        Ok(Token::Number)
    }

    fn string(&mut self) -> LexerResult {
        let mut value = String::new();

        self.next_char(); // "

        loop {
            match self.curr {
                Some('\\') => {
                    match self.next_char() {
                        Some('\\') => value.push('\\'),
                        Some('\'') => value.push('\''),
                        Some('\"') => value.push('\"'),
                        Some('n')  => value.push('\n'),
                        Some('r')  => value.push('\r'),
                        Some('t')  => value.push('\t'),
                        Some('a')  => value.push('\x07'),
                        Some('b')  => value.push('\x08'),
                        Some('f')  => value.push('\x0c'),
                        Some('v')  => value.push('\x0b'),
                        Some(chr)  => {
                            value.push('\\');
                            value.push(chr);
                        }
                        None => value.push('\\'),
                    }
                }
                Some('\"') => {
                    break;
                }
                Some('\n')
              | None => {
                    return Err(
                        MachinaError {
                            kind: MachinaErrorKind::UnterminatedString, line: self.line
                        }
                    );
                }

                _ => {}
            }

            value.push(self.next_char().unwrap());
        }

        self.next_char(); // "

        self.value = Some(value.into());

        Ok(Token::String)
    }

    fn comment(&mut self) {
        loop {
            if matches!(self.curr, Some('\n') | None) {
                break;
            }
            self.next_char();
        }
    }

    fn space(&mut self) {
        while matches!(self.curr, Some(' ') | Some('\t') | Some('\r')) {
            self.next_char();
        }
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn take_value(&mut self) -> Option<String> {
        self.value.take()
    }
}

impl<'s> Iterator for Lexer<'s> {
    type Item = LexerResult;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_token())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    // symbols
    LParen,   // (
    RParen,   // )
    LBrace,   // {
    RBrace,   // }
    LBracket, // [
    RBracket, // ]
    Comma,    // ,

    // instructions
    Call,
    Ret,
    Move,
    Jmp,
    Jt,
    Jf,
    JLt,
    JLe,
    JGt,
    JGe,
    JEq,
    JNe,
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Not,
    And,
    Or,
    Xor,
    Shl,
    Shr,
    Write,

    // values
    String,
    Number,
    Label,
    Function,
    Register,

    // descriptor
    Operand,
    Instruction,

    // others
    EOL,
    EOF,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),
            Token::LBracket => write!(f, "["),
            Token::RBracket => write!(f, "]"),
            Token::Comma => write!(f, ","),
            Token::Call => write!(f, "call"),
            Token::Ret => write!(f, "ret"),
            Token::Move => write!(f, "move"),
            Token::Jmp => write!(f, "jmp"),
            Token::Jt => write!(f, "jt"),
            Token::Jf => write!(f, "jf"),
            Token::JLt => write!(f, "jlt"),
            Token::JLe => write!(f, "jle"),
            Token::JGt => write!(f, "jgt"),
            Token::JGe => write!(f, "jge"),
            Token::JEq => write!(f, "jeq"),
            Token::JNe => write!(f, "jne"),
            Token::Lt => write!(f, "lt"),
            Token::Le => write!(f, "le"),
            Token::Gt => write!(f, "gt"),
            Token::Ge => write!(f, "ge"),
            Token::Eq => write!(f, "eq"),
            Token::Ne => write!(f, "ne"),
            Token::Add => write!(f, "add"),
            Token::Sub => write!(f, "sub"),
            Token::Mul => write!(f, "mul"),
            Token::Div => write!(f, "div"),
            Token::Mod => write!(f, "mod"),
            Token::Not => write!(f, "not"),
            Token::And => write!(f, "and"),
            Token::Or => write!(f, "or"),
            Token::Xor => write!(f, "xor"),
            Token::Shl => write!(f, "shl"),
            Token::Shr => write!(f, "shr"),
            Token::Write => write!(f, "write"),
            Token::String => write!(f, "string"),
            Token::Number => write!(f, "number"),
            Token::Label => write!(f, "label"),
            Token::Function => write!(f, "function"),
            Token::Register => write!(f, "register"),
            Token::Operand => write!(f, "operand"),
            Token::Instruction => write!(f, "instruction"),
            Token::EOL => write!(f, "end of line"),
            Token::EOF => write!(f, "end of file"),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn next_token(lexer: &mut Lexer) -> (Token, Option<String>) {
        (lexer.next().unwrap().unwrap(), lexer.take_value())
    }

    #[test]
    fn lex_instruction() {
        let mut lexer = Lexer::new("CALL");

        let (call, _) = next_token(&mut lexer);

        assert_eq!(call, Token::Call);
    }

    #[test]
    fn lex_register() {
        let mut lexer = Lexer::new("MOVE %0, 1");

        let _ = lexer.next();
        let (reg, num) = next_token(&mut lexer);

        assert_eq!(reg, Token::Register);
        assert_eq!(num, Some("0".into()));
    }

    #[test]
    fn lex_label() {
        let mut lexer = Lexer::new(".L0");

        let (label, name) = next_token(&mut lexer);

        assert_eq!(label, Token::Label);
        assert_eq!(name, Some("L0".into()));
    }

    #[test]
    fn lex_function() {
        let mut lexer = Lexer::new("@entrypoint");

        let (fun, name) = next_token(&mut lexer);

        assert_eq!(fun, Token::Function);
        assert_eq!(name, Some("entrypoint".into()));
    }

    #[test]
    fn lex_complete_instruction() {
        let source = "ADD %0, 1";
        let mut lexer = Lexer::new(&source);

        let (add, _) = next_token(&mut lexer);
        let (reg, reg_value) = next_token(&mut lexer);
        let _ = lexer.next();
        let (num, num_value) = next_token(&mut lexer);

        assert_eq!(add, Token::Add);
        assert_eq!(reg, Token::Register);
        assert_eq!(reg_value, Some("0".into()));
        assert_eq!(num, Token::Number);
        assert_eq!(num_value, Some("1".into()));
    }

    #[test]
    fn lex_number() {
        let source = "MOVE %0, 42";
        let mut lexer = Lexer::new(&source);

        let _ = lexer.next();
        let _ = lexer.next();
        let _ = lexer.next();
        let (number, number_value) = next_token(&mut lexer);

        assert_eq!(number, Token::Number);
        assert_eq!(number_value, Some("42".into()));
    }

    #[test]
    fn lex_float_number() {
        let source = "MOVE %0, 3.14519";
        let mut lexer = Lexer::new(&source);

        let _ = lexer.next();
        let _ = lexer.next();
        let _ = lexer.next();
        let (number, number_value) = next_token(&mut lexer);

        assert_eq!(number, Token::Number);
        assert_eq!(number_value, Some("3.14519".into()));
    }

    #[test]
    fn lex_simple_string() {
        let source = "MOVE %0, \"Hello, World\"";
        let mut lexer = Lexer::new(&source);

        let _ = lexer.next();
        let _ = lexer.next();
        let _ = lexer.next();
        let (string, string_value) = next_token(&mut lexer);

        assert_eq!(string, Token::String);
        assert_eq!(string_value, Some("Hello, World".into()));
    }

    #[test]
    fn lex_complex_string() {
        let source = r#"MOVE %0, "MOVE %0, \"MOVE...\"""#;
        let mut lexer = Lexer::new(&source);

        let _ = lexer.next();
        let _ = lexer.next();
        let _ = lexer.next();
        let (string, string_value) = next_token(&mut lexer);

        assert_eq!(string, Token::String);
        assert_eq!(string_value, Some(r#"MOVE %0, \"MOVE...\""#.into()));
    }

    #[test]
    fn lex_complete() {
        let source = r#"

            @entrypoint
              MOVE  %0, 1
              MOVE  %1, 2
              ADD   %0, %1
              RET   %0
        "#;

        let mut lexer = Lexer::new(&source);

        let mut tokens = vec![];

        loop {
            let (token, value) = next_token(&mut lexer);

            if token == Token::EOF {
                break;
            }

            tokens.push((token, value));
        }

        let kinds: Vec<Token> = tokens
            .iter()
            .map(|(token, _)| *token)
            .collect();

        assert_eq!(kinds, vec![
            Token::Function,
            Token::EOL,
            Token::Move,
            Token::Register,
            Token::Comma,
            Token::Number,
            Token::EOL,
            Token::Move,
            Token::Register,
            Token::Comma,
            Token::Number,
            Token::EOL,
            Token::Add,
            Token::Register,
            Token::Comma,
            Token::Register,
            Token::EOL,
            Token::Ret,
            Token::Register,
            Token::EOL,
        ]);
    }
}
