use lex::{ Token, TokenKind, get_integer };
use parse::{ expr, TokenIter };

#[derive (Debug)]
pub struct Expr {
    // @TODO other kinds of literal (make an enum)
    value: i32,
}

impl Expr {
    pub fn parse<'a, 'b: 'a, I>(
        tokens: &mut TokenIter<'a, 'b, I>,
    ) -> Option<Self> where I: Iterator<Item=&'a Token<'b>> {
        let lit = tokens.eat(TokenKind::LitInteger)?;

        Some(Expr {
            value: get_integer(lit.source)?,
        })
    }
}

impl expr::Expr for Expr {}
