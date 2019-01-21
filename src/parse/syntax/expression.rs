//! `Syntax` representing an evaluable, functional expression, i.e., bearing a
//! value.

use lex::{ Token, TokenKind };
use parse::{ syntax, TokenIter };

#[derive (Debug)]
pub struct Syntax {
    pub value: Box<syntax::Syntax>,
}

impl Syntax {
    pub fn parse<'a, 'b: 'a, I>(
        tokens: &mut TokenIter<'a, 'b, I>,
    ) -> Option<Self> where I: Iterator<Item=&'a Token<'b>> {
        // First handle a primitive (literal, variable, call...). Then handle
        // possible binary ops.

        // Eat a primitive
        let primitive = match tokens.peek()? {
            Token { kind: TokenKind::LitInteger, .. } => {
                Box::new(
                    syntax::literal::Syntax::parse(tokens)?
                ) as Box<syntax::Syntax>
            },
            Token { kind: TokenKind::OthName, .. } => {
                Box::new(
                    syntax::name::Syntax::parse(tokens)?
                ) as Box<syntax::Syntax>
            },

            // @TODO calls, ...? Blocks and other flow constructs?

            _ => return None,
        };

        // Handle a possible binary op.
        //
        // binary::Syntax will handle any more in a row - it has to, because
        // only it can swap around its members based on precedence.
        //
        // @TODO where is validity of operands to be checked?
        //
        match tokens.peek()?.kind {
            TokenKind::OpAdd |
            TokenKind::OpSub |
            TokenKind::OpMul => {
                // @OPTION None-ing out is not helpful
                let binary = syntax::binary::Syntax::parse(
                    tokens, primitive,
                )?;

                Some(Syntax { value: Box::new(binary) })
            },

            // We only peeked; it's fine if it's not a binary op.
            _ => Some(Syntax { value: primitive }),
        }
    }
}

impl syntax::Syntax for Syntax {
    fn any(&self) -> Option<&std::any::Any> { Some(self) }
    fn any_mut(&mut self) -> Option<&mut std::any::Any> { Some(self) }
}
