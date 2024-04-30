use crate::{
    ast::{DeleteStatement, Expression, SQLStatement, Value, WhereClause},
    token::Token,
    types::DBError,
};

use super::parser::{Parser, SQLParser};

struct DeleteStatementParser;
impl<P, ERR> SQLParser<P, ERR> for DeleteStatementParser
where
    P: Parser,
    ERR: DBError,
{
    fn parse(parser: &mut P) -> Result<SQLStatement, ERR> {
        if parser.peek_token() == Some(&Token::Delete) {
            parser.consume_token();
            if parser.next_token() == Some(Token::From) {
                if let Some(Token::Identifier(table_name)) = parser.next_token() {
                    if let Some(token) = parser.next_token() {
                        match token {
                            Token::Semicolon => {
                                return Ok(SQLStatement::Delete(DeleteStatement::new(
                                    table_name, None,
                                )))
                            }
                            Token::Where => {
                                if let Some(Token::Identifier(column)) = parser.next_token() {
                                    if let Some(Token::Equal) = parser.next_token() {
                                        if let Some(Token::StringLiteral(value)) =
                                            parser.next_token()
                                        {
                                            return Ok(SQLStatement::Delete(DeleteStatement::new(
                                                table_name,
                                                Some(WhereClause::new(Expression::new(
                                                    column,
                                                    Value::StrValue(value),
                                                ))),
                                            )));
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        return Err(ERR::cause("parse fail delete"));
    }
}

mod tests {
    use crate::{
        ast::SQLStatement,
        parser::{
            delete::DeleteStatementParser,
            parser::{ParserError, SQLParser, SimpleParser},
        },
        token::Token,
    };

    #[test]
    fn test() {
        let tokens = vec![
            Token::Delete,
            Token::From,
            Token::Identifier("my_table".to_string()),
            Token::Semicolon,
        ];
        let mut parser = SimpleParser::new(tokens);
        let ast: Result<SQLStatement, ParserError> = DeleteStatementParser::parse(&mut parser);
        println!("{:?}", ast.unwrap());
    }
    #[test]
    fn test2() {
        let tokens = vec![
            Token::Delete,
            Token::From,
            Token::Identifier("my_table".to_string()),
            Token::Where,
            Token::Identifier("column_name".to_string()),
            Token::Equal,
            Token::StringLiteral("value".to_string()),
            Token::Semicolon,
        ];
        let mut parser = SimpleParser::new(tokens);
        let ast: Result<SQLStatement, ParserError> = DeleteStatementParser::parse(&mut parser);
        println!("{:?}", ast.unwrap());
    }
}
