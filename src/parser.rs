use crate::token::Token;


trait Parser<ERR> {
    fn parse(&mut self) -> Result<AstNode, ERR>;
    fn next_token(&mut self) -> Option<Token>;
    fn peek_token(&self) -> Option<&Token>;
    fn consume_token(&mut self) -> bool;
}