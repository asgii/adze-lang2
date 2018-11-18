use lex::{ Token, TokenKind };
use parse::{ syntax, TokenIter };

#[derive (Debug)]
pub struct Syntax {
    signature: syntax::signature::Syntax,
    block: syntax::block::Syntax,
}

impl Syntax {
    pub fn parse<'a, 'b: 'a, I>(
        tokens: &mut TokenIter<'a, 'b, I>,
    ) -> Option<Self> where I: Iterator<Item=&'a Token<'b>> {
        let signature = syntax::signature::Syntax::parse(tokens)?;
        let block = syntax::block::Syntax::parse(tokens)?;

        Some(Syntax {
            signature,
            block,
        })
    }
}

impl syntax::Syntax for Syntax {

}