//! A `statement::Syntax` currently acts as a stand-in for the kind of
//! expressions that appear within (any) block scope. That includes more blocks,
//! if-clauses, etc.; in effect, it is an unqualified 'expression' (but that
//! term is clearly already in use to describe `Syntax`es).

use lex::{ Token, TokenKind };
use parse::{ syntax, TokenIter };

#[derive (Debug)]
pub struct Syntax {
    block: Box<syntax::Syntax>,
}

impl Syntax {
    pub fn parse<'a, 'b: 'a, I>(
        tokens: &mut TokenIter<'a, 'b, I>,
    ) -> Option<Self> where I: Iterator<Item=&'a Token<'b>> {
        // @OPTION don't return directly, put in Syntax.
        // This will be necessary if (as it probably should) parse() returns
        // statement::Syntax specifically.
        let syntax = match tokens.peek()? {
            Token { kind: TokenKind::BraceOpen, .. } => {
                Some(Syntax {
                    block: Box::new(syntax::block::Syntax::parse(tokens)?),
                })
            },
            Token { kind: TokenKind::OthName, .. } => {
                Some(Syntax {
                    block: Box::new(syntax::init::Syntax::parse(tokens)?),
                })
            },

            // @TODO other cases: if-clauses, etc.
            // @OPTION in cases where it's ambiguous, may want to do the
            // switching on parse() returns here

            _ => None,
        };

        syntax
    }
}

impl syntax::Syntax for Syntax {

}
