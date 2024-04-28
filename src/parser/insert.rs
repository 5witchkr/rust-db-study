use crate::{
    ast::{InsertStatement, SQLStatement, Value},
    token::Token,
    types::DBError,
};

use super::parser::{Parser, SQLParser};

struct InsertStatementParser;
impl<P, ERR> SQLParser<P, ERR> for InsertStatementParser
where
    P: Parser,
    ERR: DBError,
{
    fn parse(parser: &mut P) -> Result<SQLStatement, ERR> {
        if parser.peek_token() == Some(&Token::Insert) {
            parser.consume_token();
            if parser.peek_token() == Some(&Token::Into) {
                parser.consume_token();
            }
            if let Some(Token::Identifier(table_name)) = parser.next_token() {
                parser.consume_token();
                let mut columns = Vec::new();
                loop {
                    if let Some(Token::Identifier(name)) = parser.next_token() {
                        columns.push(name);
                        if parser.peek_token() == Some(&Token::Comma) {
                            parser.consume_token();
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                if parser.peek_token() == Some(&Token::RightParen) {
                    parser.consume_token();
                    if let Some(Token::Values) = parser.next_token() {
                        parser.consume_token();
                        let mut values = Vec::new();
                        loop {
                            if let Some(Token::StringLiteral(value)) = parser.next_token() {
                                values.push(Value::new(value));
                                if parser.peek_token() == Some(&Token::Comma) {
                                    parser.consume_token();
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                        if parser.peek_token() == Some(&Token::RightParen) {
                            return Ok(SQLStatement::Insert(InsertStatement::new(
                                table_name, columns, values,
                            )));
                        }
                    }
                }
            }
        }
        Err(ERR::cause("parse fail insert"))
    }
}

mod tests {
    use crate::{
        ast::SQLStatement,
        parser::{
            insert::InsertStatementParser,
            parser::{ParserError, SQLParser, SimpleParser},
        },
        token::Token,
    };

    #[test]
    fn test() {
        let tokens = vec![
            Token::Insert,
            Token::Into,
            Token::Identifier("my_table".to_string()),
            Token::LeftParen,
            Token::Identifier("id".to_string()),
            Token::Comma,
            Token::Identifier("name".to_string()),
            Token::RightParen,
            Token::Values,
            Token::LeftParen,
            Token::StringLiteral("id value".to_string()),
            Token::Comma,
            Token::StringLiteral("name value".to_string()),
            Token::RightParen,
            Token::Semicolon,
        ];
        let mut parser = SimpleParser::new(tokens);
        let ast: Result<SQLStatement, ParserError> = InsertStatementParser::parse(&mut parser);
        println!("{:?}", ast.unwrap());
    }
}
