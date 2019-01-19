use lex::{ Token, TokenKind, get_integer };
use parse::{ syntax, syntax::SyntaxKind, TokenIter };

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
    fn kind(&self) -> SyntaxKind { SyntaxKind::Literal }
}
