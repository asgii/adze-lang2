//! A `statement::Expr` currently acts as a stand-in for the kind of expressions
//! that appear within (any) block scope. That includes more blocks, if-clauses,
//! etc.; in effect, it is an unqualified 'expression' (but that term is clearly
//! already in use to describe `Expr`s).

use lex::{ Token, TokenKind };
use parse::expr;

#[derive (Debug)]
pub struct Expr {
    block: Box<expr::Expr>,
}

impl Expr {
    pub fn parse<'a, 'b: 'a, I>(
        tokens: &mut I
    ) -> Option<Self> where I: Iterator<Item=&'a Token<'b>> {
        let token = tokens.next()?;
        // @OPTION don't return directly, put in Expr.
        // This will be necessary if (as it probably should) parse() returns
        // statement::Expr specifically.
        match token {
            Token { kind: TokenKind::BraceOpen, .. } => {
                return Some(Expr {
                    block: Box::new(expr::block::Expr::parse(tokens)?),
                });
            },

            // @TODO other cases: if-clauses, etc.
            // @OPTION in cases where it's ambiguous, may want to do the
            // switching on parse() returns here

            _ => return None,
        }
    }
}

impl expr::Expr for Expr {

}
