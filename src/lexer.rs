use std::marker::PhantomData;

use crate::{token::Token, types::DBError};

pub trait Tokenize<ERR> {
    fn tokenize(input: String) -> Result<Vec<Token>, ERR>;
}
trait Lexer<ERR> {
    fn new(input: String) -> Self;
    fn read_char(&mut self);
    fn next_token(&mut self) -> Result<Token, ERR>;
    fn read_identifier(&mut self) -> String;
    fn read_number(&mut self) -> f64;
    fn skip_whitespace(&mut self);
    fn lookup_identifier(&self, identifier: &str) -> Token;
}

struct LexerError(String);
impl DBError for LexerError {
    fn cause(msg: String) -> Self {
        LexerError(msg)
    }
}

struct SimpleLexer<ERR> {
    chars: Vec<char>,
    index: usize,
    char: char,
    _err: PhantomData<ERR>,
}
impl<ERR> Tokenize<ERR> for SimpleLexer<ERR>
where
    ERR: DBError,
{
    fn tokenize(input: String) -> Result<Vec<Token>, ERR> {
        let mut lexer = SimpleLexer::new(input);
        let mut tokens = Vec::new();
        loop {
            let token = lexer.next_token()?;
            if token.is_eof() {
                return Ok(tokens);
            }
            tokens.push(token);
        }
    }
}
impl<ERR> Lexer<ERR> for SimpleLexer<ERR> {
    fn new(input: String) -> Self {
        let mut lexer = SimpleLexer {
            chars: input.chars().collect(),
            index: 0,
            char: '\0',
            _err: PhantomData,
        };
        lexer.read_char();
        lexer
    }
    fn read_char(&mut self) {
        self.char = self.chars.get(self.index).map_or('\0', |v| *v);
        self.index += 1;
    }

    fn next_token(&mut self) -> Result<Token, ERR> {
        todo!()
    }

    fn read_identifier(&mut self) -> String {
        todo!()
    }

    fn read_number(&mut self) -> f64 {
        todo!()
    }

    fn skip_whitespace(&mut self) {
        while self.char.is_whitespace() {
            self.read_char();
        }
    }

    fn lookup_identifier(&self, identifier: &str) -> Token {
        todo!()
    }
}

mod tests {
    use std::{hint::black_box, process::Termination, time::Instant};

    use super::{Lexer, LexerError, SimpleLexer};

    #[test]
    fn ptest_read() {
        let start = Instant::now();
        for _ in 0..1_000_000 {
            let mut lexer: SimpleLexer<LexerError> = SimpleLexer::new("1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890".to_string());
            let mut reulst_vec = Vec::new();
            for _ in 0..100 {
                lexer.read_char();
                let c = lexer.char;
                black_box(reulst_vec.push(c));
            }
        }
        println!("FINSIH: {:?}", start.elapsed());
    }
}
