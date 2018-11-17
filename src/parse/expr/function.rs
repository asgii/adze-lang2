use lex::{ Token, TokenKind };
use parse::expr;

pub struct Expr {
    // @TODO should these have type information?
    // Some expressions must have members that are type-generic: ones that
    // bracket multiple possible ones together.
    signature: Box<expr::Expr>,
    block: Box<expr::Expr>,
}

impl Expr {
    pub fn parse<'a, 'b: 'a, I>(
        tokens: &mut I
    ) -> Option<Self> where I: Iterator<Item=&'a Token<'b>> {
        let signature = expr::signature::Expr::parse(tokens)?;
        let block = expr::block::Expr::parse(tokens)?;

        Some(Expr {
            signature: Box::new(signature),
            block: Box::new(block),
        })
    }
}

impl expr::Expr for Expr {

}