use lex::{ Token, TokenKind };
use parse::{ syntax, TokenIter };

#[derive (Debug)]
pub struct Syntax {
    // @OPTION eventually Syntaxs will probably have &Tokens, at which point you
    // already have the name
    string: String,
}

impl Syntax {
    pub fn parse<'a, 'b: 'a, I>(
        tokens: &mut TokenIter<'a, 'b, I>,
    ) -> Option<Self> where I: Iterator<Item=&'a Token<'b>> {
        // @TODO check validity as variable name
        let name = tokens.eat(TokenKind::OthName)?;

        Some(Syntax {
            string: name.source.to_string(),
        })
    }
}

impl syntax::Syntax for Syntax {
    fn any(&self) -> Option<&std::any::Any> { Some(self) }
    fn any_mut(&mut self) -> Option<&mut std::any::Any> { Some(self) }
}
