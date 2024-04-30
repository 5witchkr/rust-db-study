use core::num;
use std::io::stdout;

use crate::{
    ast::{Expression, SQLStatement, SetClause, UpdateStatement, Value, WhereClause},
    token::Token,
    types::DBError,
};

use super::parser::{Parser, SQLParser};

struct UpdateStatementParser;
impl<P, ERR> SQLParser<P, ERR> for UpdateStatementParser
where
    P: Parser,
    ERR: DBError,
{
    fn parse(parser: &mut P) -> Result<crate::ast::SQLStatement, ERR> {
        if parser.peek_token() == Some(&Token::Update) {
            parser.consume_token();
            if let Some(Token::Identifier(table_name)) = parser.next_token() {
                if parser.peek_token() == Some(&Token::Set) {
                    parser.consume_token();
                    let mut set_clauses: Vec<SetClause> = Vec::new();
                    loop {
                        if parser.peek_token() == Some(&Token::Where) {
                            break;
                        }
                        if let Some(token) = parser.next_token() {
                            match token {
                                Token::Identifier(column_name) => {
                                    if parser.peek_token() == Some(&Token::Equal) {
                                        parser.consume_token();
                                        if let Some(token) = parser.next_token() {
                                            match token {
                                                Token::StringLiteral(str_value) => set_clauses
                                                    .push(SetClause::new(
                                                        column_name,
                                                        Value::StrValue(str_value),
                                                    )),
                                                Token::NumericLiteral(num_value) => set_clauses
                                                    .push(SetClause::new(
                                                        column_name,
                                                        Value::NumValue(num_value),
                                                    )),
                                                _ => break,
                                            }
                                        }
                                    }
                                }
                                Token::Comma => {}
                                _ => break,
                            }
                        } else {
                            break;
                        }
                    }
                    if parser.peek_token() == Some(&Token::Where) {
                        parser.consume_token();
                        if let Some(Token::Identifier(column)) = parser.next_token() {
                            if parser.peek_token() == Some(&Token::Equal) {
                                parser.consume_token();
                                if let Some(value_token) = parser.next_token() {
                                    match value_token {
                                        Token::StringLiteral(str_value) => {
                                            return Ok(SQLStatement::Update(UpdateStatement::new(
                                                table_name,
                                                set_clauses,
                                                Some(WhereClause::new(Expression::new(
                                                    column,
                                                    Value::StrValue(str_value),
                                                ))),
                                            )))
                                        }
                                        Token::NumericLiteral(num_value) => {
                                            return Ok(SQLStatement::Update(UpdateStatement::new(
                                                table_name,
                                                set_clauses,
                                                Some(WhereClause::new(Expression::new(
                                                    column,
                                                    Value::NumValue(num_value),
                                                ))),
                                            )))
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                    } else {
                        return Ok(SQLStatement::Update(UpdateStatement::new(
                            table_name,
                            set_clauses,
                            None,
                        )));
                    }
                }
            }
        }
        Err(ERR::cause("msg"))
    }
}

mod tests {
    use crate::{
        ast::SQLStatement,
        parser::{
            parser::{ParserError, SQLParser, SimpleParser},
            update::UpdateStatementParser,
        },
        token::Token,
    };

    #[test]
    fn test() {
        struct TestCase {
            tokens: Vec<Token>,
        }
        let test_cases = vec![
            TestCase {
                tokens: vec![
                    Token::Update,
                    Token::Identifier("my_table".to_string()),
                    Token::Set,
                    Token::Identifier("name".to_string()),
                    Token::Equal,
                    Token::StringLiteral("name_value".to_string()),
                    Token::Comma,
                    Token::Identifier("city".to_string()),
                    Token::Equal,
                    Token::StringLiteral("city_value".to_string()),
                    Token::Where,
                    Token::Identifier("id".to_string()),
                    Token::Equal,
                    Token::NumericLiteral(1.0),
                    Token::Semicolon,
                ],
            },
            TestCase {
                tokens: vec![
                    Token::Update,
                    Token::Identifier("my_table".to_string()),
                    Token::Set,
                    Token::Identifier("name".to_string()),
                    Token::Equal,
                    Token::StringLiteral("name_value".to_string()),
                    Token::Comma,
                    Token::Identifier("city".to_string()),
                    Token::Equal,
                    Token::StringLiteral("city_value".to_string()),
                    Token::Semicolon,
                ],
            },
            TestCase {
                tokens: vec![
                    Token::Update,
                    Token::Identifier("my_table".to_string()),
                    Token::Set,
                    Token::Identifier("name".to_string()),
                    Token::Equal,
                    Token::StringLiteral("name_value".to_string()),
                    Token::Where,
                    Token::Identifier("id".to_string()),
                    Token::Equal,
                    Token::StringLiteral("uuid".to_string()),
                    Token::Semicolon,
                ],
            },
        ];
        for case in test_cases {
            let mut parser = SimpleParser::new(case.tokens);
            let ast: Result<SQLStatement, ParserError> = UpdateStatementParser::parse(&mut parser);
            println!("{:?}", ast.unwrap());
        }
    }
}
