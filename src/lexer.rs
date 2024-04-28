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
    fn read_value(&mut self) -> String;
    fn read_number(&mut self) -> Result<f64, ERR>;
    fn skip_whitespace(&mut self);
    fn lookup_identifier(&self, identifier: &str) -> Token;
}

#[derive(Debug)]
struct LexerError(String);
impl DBError for LexerError {
    fn cause(msg: &str) -> Self {
        LexerError(msg.to_string())
    }
    fn and_cause(mut self, msg: &str) -> Self {
        self.0.push_str(msg);
        self
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
impl<ERR> Lexer<ERR> for SimpleLexer<ERR>
where
    ERR: DBError,
{
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
        self.skip_whitespace();
        let token = match self.char {
            '=' => Token::Equal,
            ';' => Token::Semicolon,
            ',' => Token::Comma,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '*' => Token::Asterisk,
            '\0' => Token::EOF,
            '"' | '\'' => Token::StringLiteral(self.read_value()),
            _ => {
                if self.char.is_alphabetic() {
                    let identifier = self.read_identifier();
                    return Ok(self.lookup_identifier(&identifier));
                } else if self.char.is_numeric() {
                    return Ok(Token::NumericLiteral(self.read_number()?));
                } else {
                    Token::Unknown(self.char)
                }
            }
        };
        self.read_char();
        Ok(token)
    }

    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();
        while self.char.is_alphanumeric() || self.char == '_' {
            identifier.push(self.char);
            self.read_char();
        }
        identifier
    }
    fn read_value(&mut self) -> String {
        let mut string_val = String::new();
        self.read_char();
        while self.char != '"' && self.char != '\'' && self.char != '\0' {
            string_val.push(self.char);
            self.read_char();
        }
        string_val
    }
    fn read_number(&mut self) -> Result<f64, ERR> {
        let mut number_str = String::new();
        while self.char.is_digit(10) || self.char == '.' {
            number_str.push(self.char);
            self.read_char();
        }
        number_str
            .parse::<f64>()
            .map_err(|_| ERR::cause("error read_number"))
    }

    fn skip_whitespace(&mut self) {
        while self.char.is_whitespace() {
            self.read_char();
        }
    }

    fn lookup_identifier(&self, identifier: &str) -> Token {
        match identifier.to_uppercase().as_str() {
            "TABLE" => Token::Table,
            "CREATE" => Token::Create,
            "DROP" => Token::Drop,
            "INSERT" => Token::Insert,
            "SELECT" => Token::Select,
            "UPDATE" => Token::Update,
            "DELETE" => Token::Delete,
            "INTO" => Token::Into,
            "FROM" => Token::From,
            "WHERE" => Token::Where,
            "SET" => Token::Set,
            "VALUES" => Token::Values,
            _ => Token::Identifier(identifier.to_string()),
        }
    }
}

mod tests {
    use std::{hint::black_box, process::Termination, time::Instant};

    use crate::token::Token;

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

    #[test]
    fn test_tokenize() {
        struct TestCase {
            input: String,
            expected_tokens: Vec<Token>,
        }
        let test_cases = vec![
            TestCase {
                input: "SELECT * FROM table_name WHERE column_name = 'value';".to_owned(),
                expected_tokens: vec![
                    Token::Select,
                    Token::Asterisk,
                    Token::From,
                    Token::Identifier("table_name".to_string()),
                    Token::Where,
                    Token::Identifier("column_name".to_string()),
                    Token::Equal,
                    Token::StringLiteral("value".to_string()),
                    Token::Semicolon,
                    Token::EOF,
                ],
            },
            TestCase {
                input: "var_name = 123;".to_owned(),
                expected_tokens: vec![
                    Token::Identifier("var_name".to_string()),
                    Token::Equal,
                    Token::NumericLiteral(123.0),
                    Token::Semicolon,
                    Token::EOF,
                ],
            },
            TestCase {
                input: "CREATE TABLE users (id, name);".to_owned(),
                expected_tokens: vec![
                    Token::Create,
                    Token::Table,
                    Token::Identifier("users".to_string()),
                    Token::LeftParen,
                    Token::Identifier("id".to_string()),
                    Token::Comma,
                    Token::Identifier("name".to_string()),
                    Token::RightParen,
                    Token::Semicolon,
                    Token::EOF,
                ],
            },
            TestCase {
                input: "SELECT * FROM users;".to_owned(),
                expected_tokens: vec![
                    Token::Select,
                    Token::Asterisk,
                    Token::From,
                    Token::Identifier("users".to_string()),
                    Token::Semicolon,
                    Token::EOF,
                ],
            },
            TestCase {
                input: "UPDATE users SET name = 'John Doe' WHERE id = 1;".to_owned(),
                expected_tokens: vec![
                    Token::Update,
                    Token::Identifier("users".to_string()),
                    Token::Set,
                    Token::Identifier("name".to_string()),
                    Token::Equal,
                    Token::StringLiteral("John Doe".to_string()),
                    Token::Where,
                    Token::Identifier("id".to_string()),
                    Token::Equal,
                    Token::NumericLiteral(1.0),
                    Token::Semicolon,
                    Token::EOF,
                ],
            },
            TestCase {
                input: "DELETE FROM users WHERE id = 1;".to_owned(),
                expected_tokens: vec![
                    Token::Delete,
                    Token::From,
                    Token::Identifier("users".to_string()),
                    Token::Where,
                    Token::Identifier("id".to_string()),
                    Token::Equal,
                    Token::NumericLiteral(1.0),
                    Token::Semicolon,
                    Token::EOF,
                ],
            },
            TestCase {
                input: "INSERT INTO table_name (column1, column2) VALUES ('value1', 123);"
                    .to_owned(),
                expected_tokens: vec![
                    Token::Insert,
                    Token::Into,
                    Token::Identifier("table_name".to_string()),
                    Token::LeftParen,
                    Token::Identifier("column1".to_string()),
                    Token::Comma,
                    Token::Identifier("column2".to_string()),
                    Token::RightParen,
                    Token::Values,
                    Token::LeftParen,
                    Token::StringLiteral("value1".to_string()),
                    Token::Comma,
                    Token::NumericLiteral(123.0),
                    Token::RightParen,
                    Token::Semicolon,
                    Token::EOF,
                ],
            },
            TestCase {
                input: "SELECT column1 FROM table_name;".to_owned(),
                expected_tokens: vec![
                    Token::Select,
                    Token::Identifier("column1".to_string()),
                    Token::From,
                    Token::Identifier("table_name".to_string()),
                    Token::Semicolon,
                    Token::EOF,
                ],
            },
            TestCase {
                input: "UPDATE table_name SET column1 = 'new_value' WHERE column2 = 123;"
                    .to_owned(),
                expected_tokens: vec![
                    Token::Update,
                    Token::Identifier("table_name".to_string()),
                    Token::Set,
                    Token::Identifier("column1".to_string()),
                    Token::Equal,
                    Token::StringLiteral("new_value".to_string()),
                    Token::Where,
                    Token::Identifier("column2".to_string()),
                    Token::Equal,
                    Token::NumericLiteral(123.0),
                    Token::Semicolon,
                    Token::EOF,
                ],
            },
            TestCase {
                input: "DELETE FROM table_name WHERE column1 = 'value1';".to_owned(),
                expected_tokens: vec![
                    Token::Delete,
                    Token::From,
                    Token::Identifier("table_name".to_string()),
                    Token::Where,
                    Token::Identifier("column1".to_string()),
                    Token::Equal,
                    Token::StringLiteral("value1".to_string()),
                    Token::Semicolon,
                    Token::EOF,
                ],
            },
        ];
        let mut case_num = 0;
        for case in test_cases {
            case_num += 1;
            println!("case: {}", case_num);
            let mut lexer: SimpleLexer<LexerError> = SimpleLexer::new(case.input);
            for expected in case.expected_tokens {
                let token = lexer.next_token().unwrap();
                assert_eq!(expected, token);
            }
        }
    }
}
