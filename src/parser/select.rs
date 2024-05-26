use crate::{
    ast::{SelectStatement, SQLStatement, Expression},
    token::Token,
    types::DBError,
};

use super::parser::{Parser, SQLParser};

struct SelectStatementParser;

impl<P, ERR> SQLParser<P, ERR> for SelectStatementParser
where
    P: Parser,
    ERR: DBError,
{
    fn parse(parser: &mut P) -> Result<SQLStatement, ERR> {
        if parser.peek_token() != Some(&Token::Select) {
            return Err(ERR::cause("Expected 'Select' token but not found"));
        }
        parser.consume_token();

        let mut columns = Vec::new();
        while let Some(token) = parser.next_token() {
            match token {
                Token::Identifier(column_name) => {
                    columns.push(column_name);
                    if parser.peek_token() != Some(&Token::Comma) {
                        break;
                    }
                    parser.consume_token(); // Consume the comma
                },
                Token::From => break,
                _ => return Err(ERR::cause("Unexpected token while expecting column name or 'From'")),
            }
        }

        if parser.next_token() != Some(Token::From) {
            return Err(ERR::cause("Expected 'From' after columns but not found"));
        }

        if let Some(Token::Identifier(table_name)) = parser.next_token() {
            Ok(SQLStatement::Select(SelectStatement::new(table_name, columns)))
        } else {
            Err(ERR::cause("Expected table name after 'From' but not found"))
        }
    }
}

mod tests {
    use crate::{
        ast::{SQLStatement, SelectStatement},
        parser::{
            parser::{ParserError, SQLParser, SimpleParser}, select::SelectStatementParser
        },
        token::Token, types::DBError,
    };

    #[test]
    fn test_select_parser() {
        let test_cases = vec![
            (
                vec![
                    Token::Select,
                    Token::Identifier("column1".to_string()),
                    Token::Comma,
                    Token::Identifier("column2".to_string()),
                    Token::From,
                    Token::Identifier("table_name".to_string()),
                    Token::Semicolon,
                ],
                Ok(SQLStatement::Select(SelectStatement::new("table_name".to_string(), vec!["column1".to_string(), "column2".to_string()])))
            ),
            (
                vec![
                    Token::Select,
                    Token::Identifier("column3".to_string()),
                    Token::From,
                    Token::Identifier("another_table".to_string()),
                    Token::Semicolon,
                ],
                Ok(SQLStatement::Select(SelectStatement::new("another_table".to_string(), vec!["column3".to_string()])))
            ),
            (
                vec![
                    Token::Select,
                    Token::Identifier("column1".to_string()),
                    Token::Comma,
                    Token::Identifier("column4".to_string()),
                    Token::Comma,
                    Token::Identifier("column5".to_string()),
                    Token::From,
                    Token::Identifier("table_name".to_string()),
                    Token::Semicolon,
                ],
                Ok(SQLStatement::Select(SelectStatement::new("table_name".to_string(), vec!["column1".to_string(), "column4".to_string(), "column5".to_string()])))
            ),
        ];

        for (tokens, expected) in test_cases {
            let mut parser = SimpleParser::new(tokens);
            let result: Result<SQLStatement, ParserError> = SelectStatementParser::parse(&mut parser);
            assert_eq!(result, expected);
            println!("{:?}", result.unwrap());
        }
    }
}

