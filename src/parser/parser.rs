use crate::{ast::SQLStatement, token::Token, types::DBError};

pub trait SQLParser<P, ERR> {
    fn parse(parser: &mut P) -> Result<SQLStatement, ERR>;
}
pub trait Parser {
    fn next_token(&mut self) -> Option<Token>;
    fn peek_token(&self) -> Option<&Token>;
    fn consume_token(&mut self) -> bool;
}

pub struct SimpleParser {
    tokens: Vec<Token>,
    position: usize,
}
impl Parser for SimpleParser {
    fn next_token(&mut self) -> Option<Token> {
        if self.position < self.tokens.len() {
            let token = self.tokens[self.position].clone();
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }

    fn peek_token(&self) -> Option<&Token> {
        if self.position < self.tokens.len() {
            Some(&self.tokens[self.position])
        } else {
            None
        }
    }

    fn consume_token(&mut self) -> bool {
        if self.position < self.tokens.len() {
            self.position += 1;
            true
        } else {
            false
        }
    }
}

impl SimpleParser {
    pub fn new(tokens: Vec<Token>) -> Self {
        SimpleParser {
            tokens,
            position: 0,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ParserError(String);
impl DBError for ParserError {
    fn cause(msg: &str) -> Self {
        ParserError(msg.to_string())
    }

    fn and_cause(mut self, msg: &str) -> Self {
        self.0.push_str(msg);
        self
    }
}
