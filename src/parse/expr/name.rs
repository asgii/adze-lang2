use lex::{ Token, TokenKind };
use parse::{ expr, TokenIter };

#[derive (Debug)]
pub struct Expr {
    // @OPTION eventually Exprs will probably have &Tokens, at which point you
    // already have the name
    string: String,
}

impl Expr {
    pub fn parse<'a, 'b: 'a, I>(
        tokens: &mut TokenIter<'a, 'b, I>,
    ) -> Option<Self> where I: Iterator<Item=&'a Token<'b>> {
        // @TODO check validity as variable name
        let name = tokens.eat(TokenKind::OthName)?;

        Some(Expr {
            string: name.source.to_string(),
        })
    }
}

impl expr::Expr for Expr {}
