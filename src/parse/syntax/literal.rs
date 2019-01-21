use lex::{ Token, TokenKind, get_integer };
use parse::{ syntax, TokenIter };

#[derive (Debug)]
pub struct Syntax {
    // @TODO other kinds of literal (make an enum)
    value: i32,
}

impl Syntax {
    pub fn parse<'a, 'b: 'a, I>(
        tokens: &mut TokenIter<'a, 'b, I>,
    ) -> Option<Self> where I: Iterator<Item=&'a Token<'b>> {
        let lit = tokens.eat(TokenKind::LitInteger)?;

        Some(Syntax {
            value: get_integer(lit.source)?,
        })
    }
}

impl syntax::Syntax for Syntax {
    fn any(&self) -> Option<&std::any::Any> { Some(self) }
    fn any_mut(&mut self) -> Option<&mut std::any::Any> { Some(self) }
}
