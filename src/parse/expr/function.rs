use lex::{ Token, TokenKind };
use parse::{ expr, TokenIter };

#[derive (Debug)]
pub struct Expr {
    signature: expr::signature::Expr,
    block: expr::block::Expr,
}

impl Expr {
    pub fn parse<'a, 'b: 'a, I>(
        tokens: &mut TokenIter<'a, 'b, I>,
    ) -> Option<Self> where I: Iterator<Item=&'a Token<'b>> {
        let signature = expr::signature::Expr::parse(tokens)?;
        let block = expr::block::Expr::parse(tokens)?;

        Some(Expr {
            signature,
            block,
        })
    }
}

impl expr::Expr for Expr {

}