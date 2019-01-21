//! A `statement::Syntax` currently acts as a stand-in for the kind of
//! expressions that appear within (any) block scope.
//!
//! That includes more blocks, `if`-clauses, etc.; in effect, it is an
//! unqualified 'expression' - but that term is likely to be used for functional
//! expressions (i.e. which have a value), whereas `statement::Syntax` is
//! currently agnostic.

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
        let block = match tokens.peek()? {
            Token { kind: TokenKind::BraceOpen, .. } => {
                Box::new(
                    syntax::block::Syntax::parse(tokens)?,
                ) as Box<syntax::Syntax>
            },

            // @TODO it should eventually be valid to leave a name at the end of
            // a block as its return (here I am only allowing inits)
            Token { kind: TokenKind::OthName, .. } => {
                Box::new(
                    syntax::init::Syntax::parse(tokens)?,
                ) as Box<syntax::Syntax>
            },

            // @TODO other cases: if-clauses, etc.
            // @OPTION in cases where it's ambiguous, may want to do the
            // switching on parse() returns here

            _ => return None,
        };

        Some(Syntax { block })
    }
}

impl syntax::Syntax for Syntax {
    fn any(&self) -> Option<&std::any::Any> { Some(self) }
    fn any_mut(&mut self) -> Option<&mut std::any::Any> { Some(self) }
}
