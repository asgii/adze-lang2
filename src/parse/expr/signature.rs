use lex::{ Token, TokenKind };
use parse::expr;

pub struct Expr {
    parameters: Vec<Box<expr::Expr>>,
}

impl Expr {
    pub fn parse<'a, 'b: 'a, I>(
        tokens:&mut I
    ) -> Option<Self> where I: Iterator<Item=&'a Token<'b>> {
        // @TODO make a lex::Tokens iterator, have a standardised expect()
        match tokens.next()? {
            Token { kind: TokenKind::ParenOpen, .. } => (),
            _ => return None,
        }

        // @TODO
        let mut parameters = Vec::new();
        /*
        let parameter_iter = std::iter::repeat_with(|| {
            expr::parameter::Expr::parse(&mut tokens)?
        });
        for parameters in parameter_iter {
            parameters.push(paramater);
        }
        */

        match tokens.next()? {
            Token { kind: TokenKind::ParenClose, .. } => (),
            _ => return None,
        }

        Some(Expr {
            parameters,
        })
    }
}

impl expr::Expr for Expr {

}
