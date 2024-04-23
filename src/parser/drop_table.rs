use crate::{ast::{DropStatement, SQLStatement}, token::Token, types::DBError};

use super::parser::{Parser, ParserError, SQLParser};


struct DropStatementParser;
impl<P, ERR> SQLParser<P, ERR> for DropStatementParser
where
    ERR: DBError,
    P: Parser {
    fn parse(parser: &mut P) -> Result<crate::ast::SQLStatement, ERR> {
        if parser.consume_token() && parser.peek_token() == Some(&Token::Table) {
            parser.consume_token();
            if let Some(Token::Identifier(table_name)) = parser.next_token() {
                return Ok(SQLStatement::DropTable(DropStatement::new(table_name)))
            }
        }
        Err(ERR::cause("parse fail drop table"))
    }
}

mod tests {
    use crate::{ast::SQLStatement, parser::{drop_table::DropStatementParser, parser::{ParserError, SQLParser, SimpleParser}}, token::Token};

#[test]
fn test() {
    let tokens = vec![
        Token::Drop,
        Token::Table,
        Token::Identifier("my_table".to_string()),
        Token::Semicolon,
    ];
    let mut parser = SimpleParser::new(tokens);
    let ast: Result<SQLStatement, ParserError> = DropStatementParser::parse(&mut parser);
    println!("{:?}", ast.unwrap());
}
}
