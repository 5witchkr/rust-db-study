#[derive(Debug, PartialEq)]
pub enum Token {
    // ddl
    Table,
    Create,
    Drop,

    // dml
    Insert,
    Select,
    Update,
    Delete,
    Into,
    From,
    Where,
    Set,
    Values,

    Equal,      // =
    Semicolon,  // ;
    Comma,      // ,
    LeftParen,  // (
    RightParen, // )
    Asterisk,   // *

    Identifier(String),
    StringLiteral(String),
    NumericLiteral(f64),

    //etc
    EOF,
    Unknown(char),
}

impl Token {
    pub fn is_eof(&self) -> bool {
        matches!(self, Token::EOF)
    }
}
