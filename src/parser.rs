// use crate::object::ObjectValue;
// 
// use std::collections::HashMap;
// 
// type ParserResult<T> = Result<T, MachinaError>;
// 
// #[derive(Debug, Clone)]
// pub struct Parser<'a> {
//     lexer: Lexer<std::str::Chars<'a>>,
//     curr: Option<Token>,
//     peek: Option<Token>
// }
// 
// impl<'a> Parser<'a> {
//     pub fn new(source: &'a str) -> Parser {
//         Parser {
//             lexer: Lexer::new(source.chars()),
//             curr: None,
//             peek: None,
//         }
//     }
// 
//     pub fn parse(&mut self) -> Result<Module, Vec<MachinaError>> {
//         let _ = self.next(); // curr
//         let _ = self.next(); // peek
//         self.parse_module()
//     }
// 
//     fn parse_module(&mut self) -> Result<Module, Vec<MachinaError>> {
//         let mut errors = vec![];
// 
//         let mut blocks = vec![];
// 
//         while self.curr.is_some() {
//             match self.parse_block() {
//                 Ok(block) => {
//                     blocks.push(block)
//                 }
//                 Err(err) => {
//                     errors.push(err);
//                     self.recover(&mut errors);
//                 }
//             }
//         }
// 
//         let mut values: HashMap<ObjectValue, usize> = HashMap::new();
//         let mut variables: HashMap<Variable, usize> = HashMap::new();
// 
//         let mut blocks_lines = HashMap::new();
//         let mut count = 0;
//         
//         for (label, pre) in blocks.iter() {
//             blocks_lines.insert(label.clone(), count);
//             count += pre.len();
//         }
// 
//         let mut instructions = vec![];
// 
//         for (_, pre_instructions) in blocks {
//             instructions.extend(
//                 pre_instructions
//                     .into_iter()
//                     .map(|i| Parser::convert_instruction(i, &mut values, &mut variables, &blocks_lines))
//             )
//         }
// 
//         let mut constants = vec![ObjectValue::Null; values.len()];
// 
//         for (obj, idx) in values {
//             constants[idx] = obj;
//         }
// 
//         let variables = variables.len();
// 
//         Ok(Module {
//             variables,
//             constants,
//             instructions,
//         })
//     }
// 
//     fn parse_block(&mut self) -> ParserResult<(Label, Vec<PreInstruction>)> {
//         let mut pre_instructions = vec![];
// 
//         let label = self.parse_label()?;
//         self.eat(TokenKind::Colon)?;
// 
//         while !self.is_block_definition()
//            && !self.curr_is(TokenKind::EOF) {
//             pre_instructions.push(self.parse_pre_instruction()?);
//         }
// 
//         Ok((label, pre_instructions))
//     }
// 
//     fn parse_pre_instruction(&mut self) -> ParserResult<PreInstruction> {
//         let line = self.line();
//         let kind = InstructionKind::from_token(self.curr_kind());
//         if  kind.is_none() {
//             return Err(self.unexpected(&[TokenKind::Instruction]));
//         }
//         self.next()?;
//         let kind = kind.unwrap();
//         let arg  = match kind {
//             InstructionKind::Const
//           | InstructionKind::Load
//           | InstructionKind::Store
//           | InstructionKind::Jump
//           | InstructionKind::JumpT
//           | InstructionKind::JumpF
//           | InstructionKind::Call => {
//             Some(self.parse_value()?)
//           }
//           _ => None
//         };
//         Ok(PreInstruction { kind, arg, line })
//     }
// 
//     fn convert_instruction(
//         instruction: PreInstruction,
//         values: &mut HashMap<ObjectValue, usize>,
//         variables: &mut HashMap<Variable, usize>,
//         blocks_lines: &HashMap<Label, usize>) -> Instruction {
//         let arg = match instruction.arg {
//             Some(Value::String(s)) => {
//                 let len = values.len();
//                 *values.entry(ObjectValue::String(s)).or_insert(len)
//             }
//             Some(Value::Integer(i)) => {
//                 let len = values.len();
//                 *values.entry(ObjectValue::Integer(i)).or_insert(len)
//             }
//             Some(Value::Decimal(d)) => {
//                 let len = values.len();
//                 *values.entry(ObjectValue::Decimal(d)).or_insert(len)
//             }
//             Some(Value::Variable(v)) => {
//                 let len = variables.len();
//                 *variables.entry(v).or_insert(len)
//             }
//             Some(Value::Label(l)) => {
//                 *blocks_lines.get(&l).unwrap()
//             }
//             None => 0
//         };
//         Instruction {
//             kind: instruction.kind,
//             line: instruction.line,
//             arg,
//         }
//     }
// 
//     fn is_block_definition(&self) -> bool {
//         self.curr_is(TokenKind::Label) && self.peek_is(TokenKind::Colon)
//     }
// 
//     fn parse_label(&mut self) -> ParserResult<Label> {
//         match self.curr.clone() {
//             Some(Token { kind: TokenKind::Label, value: Some(label), .. }) => {
//                 self.next()?;
//                 Ok(Label(label))
//             }
//             _ => Err(self.unexpected(&[TokenKind::Label])),
//         }
//     }
// 
//     fn parse_value(&mut self) -> ParserResult<Value> {
//         match self.curr.clone() {
//             Some(Token { kind: TokenKind::String, value: Some(s), .. }) => {
//                 self.next()?;
//                 Ok(Value::String(s))
//             }
//             Some(Token { kind: TokenKind::Integer, value: Some(i), .. }) => {
//                 self.next()?;
//                 let value = i.parse::<i64>().unwrap();
//                 Ok(Value::Integer(value))
//             }
//             Some(Token { kind: TokenKind::Decimal, value: Some(d), .. }) => {
//                 self.next()?;
//                 let value = d.parse::<f64>().unwrap();
//                 Ok(Value::Decimal(value))
//             }
//             Some(Token { kind: TokenKind::Label, value: Some(label), .. }) => {
//                 self.next()?;
//                 Ok(Value::Label(Label(label)))
//             }
//             Some(Token { kind: TokenKind::Variable, value: Some(variable), .. }) => {
//                 self.next()?;
//                 Ok(Value::Variable(Variable(variable)))
//             }
//             _ => Err(self.unexpected(&[
//                 TokenKind::String,
//                 TokenKind::Integer,
//                 TokenKind::Decimal,
//                 TokenKind::Label,
//                 TokenKind::Variable,
//             ])),
//         }
//     }
// 
//     fn next(&mut self) -> ParserResult<()> {
//         self.curr = self.peek.clone();
//         self.peek = if let Some(token) = self.lexer.next() {
//             Some(token?)
//         } else {
//             None
//         };
//         Ok(())
//     }
// 
//     fn eat(&mut self, tkn: TokenKind) -> ParserResult<()> {
//         if self.curr_kind() == tkn {
//             self.next()?;
//             Ok(())
//         } else {
//             Err(self.unexpected(&[tkn]))
//         }
//     }
// 
//     fn curr_kind(&self) -> TokenKind {
//         if let Some(tok) = &self.curr {
//             tok.kind
//         } else {
//             TokenKind::EOF
//         }
//     }
// 
//     fn peek_kind(&self) -> TokenKind {
//         if let Some(tok) = &self.peek {
//             tok.kind
//         } else {
//             TokenKind::EOF
//         }
//     }
// 
//     fn curr_is(&self, kind: TokenKind) -> bool {
//         self.curr_kind() == kind
//     }
// 
//     fn peek_is(&self, kind: TokenKind) -> bool {
//         self.peek_kind() == kind
//     }
// 
//     fn line(&self) -> u32 {
//         if let Some(tok) = &self.curr { tok.line } else { 0 }
//     }
// 
//     fn recover(&mut self, errors: &mut Vec<MachinaError>) {
//         while !self.is_block_definition()
//            && !self.curr_is(TokenKind::EOF) {
//             if let Err(err) = self.next() { errors.push(err) }
//         }
//     }
// 
//     fn unexpected(&mut self, expected: &[TokenKind]) -> MachinaError {
//         let tokens = expected.iter().map(|x| format!("`{}`", x)).collect();
//         let found  = format!("`{}`", self.curr_kind());
//         MachinaError {
//             kind: ErrorKind::UnexpectedToken(tokens, found),
//             line: self.line()
//         }
//     }
// }