use crate::{
    ast::{ColumnDefinition, CreateStatement, SQLStatement},
    token::Token,
    types::DBError,
};

use super::parser::{Parser, SQLParser};

struct CreateStatementParser;
impl<P, ERR> SQLParser<P, ERR> for CreateStatementParser
where
    ERR: DBError,
    P: Parser,
{
    fn parse(parser: &mut P) -> Result<SQLStatement, ERR> {
        if parser.consume_token() && parser.peek_token() == Some(&Token::Table) {
            parser.consume_token();
            if let Some(Token::Identifier(table_name)) = parser.next_token() {
                parser.consume_token();
                let mut columns = Vec::new();
                loop {
                    if let Some(Token::Identifier(name)) = parser.next_token() {
                        columns.push(ColumnDefinition::new(name));
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
                    return Ok(SQLStatement::CreateTable(CreateStatement::new(
                        table_name, columns,
                    )));
                }
            }
        }
        Err(ERR::cause("empty token parse fail"))
    }
}

mod tests {
    use crate::{
        ast::{CreateStatement, SQLStatement},
        parser::{
            create_table::CreateStatementParser,
            parser::{Parser, ParserError, SQLParser, SimpleParser},
        },
        token::Token,
    };

    #[test]
    fn test() {
        let tokens = vec![
            Token::Create,
            Token::Table,
            Token::Identifier("my_table".to_string()),
            Token::LeftParen,
            Token::Identifier("id".to_string()),
            Token::Comma,
            Token::Identifier("name".to_string()),
            Token::RightParen,
            Token::Semicolon,
        ];
        let mut parser = SimpleParser::new(tokens);
        let ast: Result<SQLStatement, ParserError> = CreateStatementParser::parse(&mut parser);
        println!("{:?}", ast.unwrap());
    }
}
