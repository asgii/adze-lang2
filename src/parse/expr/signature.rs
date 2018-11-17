use lex::{ Token, TokenKind };
use parse::{ expr, TokenIter };

#[derive (Debug)]
pub struct Expr {
    parameters: Vec<Box<expr::Expr>>,
}

impl Expr {
    pub fn parse<'a, 'b: 'a, I>(
        tokens: &mut TokenIter<'a, 'b, I>,
    ) -> Option<Self> where I: Iterator<Item=&'a Token<'b>> {
        tokens.eat(TokenKind::ParenOpen)?;

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

        tokens.eat(TokenKind::ParenClose)?;

        Some(Expr {
            parameters,
        })
    }
}

impl expr::Expr for Expr {

}
