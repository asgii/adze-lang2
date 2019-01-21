extern crate enum_map;

use lex::{ Token, TokenKind };
use parse::{ syntax, TokenIter };

lazy_static! {
    static ref PRECEDENCE: enum_map::EnumMap<TokenKind, usize> = enum_map! {
        // Greater values mean greater precedence.
        TokenKind::OpAdd => 1,
        TokenKind::OpSub => 1,
        TokenKind::OpMul => 2,

        // @OPTION enum_map requires exhaustive pattern. Wrap
        _ => 1,
    };
}

#[derive (Debug)]
pub struct Syntax {
    lhs: Box<syntax::Syntax>,
    rhs: Box<syntax::Syntax>,
    op: TokenKind,
}

// @TODO handle associativity

impl Syntax {
    /// Must be passed an `lhs` but started on the binary op's token itself.
    pub fn parse<'a, 'b: 'a, I>(
        tokens: &mut TokenIter<'a, 'b, I>,
        mut lhs: Box<syntax::Syntax>,
    ) -> Option<Self> where I: Iterator<Item=&'a Token<'b>> {
        // peek() and then eat() is a circuitous way of doing next().
        //
        // @OPTION here and elsewhere it's irritating doing all of them. Only
        // have one Token, and then have helpers?
        //
        let op = match tokens.peek()? {
            Token { kind: TokenKind::OpAdd, .. } => TokenKind::OpAdd,
            Token { kind: TokenKind::OpSub, .. } => TokenKind::OpSub,
            Token { kind: TokenKind::OpMul, .. } => TokenKind::OpMul,
            _ => panic!("binary::parse() called on non-binary Token"),
        };
        tokens.eat(op).unwrap();

        // If the rhs is another binary::Syntax, precedence may require us to
        // re-order expressions from the L-R way they're read.
        //
        // Note rhs.value, because the type of the expression::Syntax is never
        // going to be binary::Syntax.
        //
        let mut rhs = Box::new(syntax::expression::Syntax::parse(tokens)?);
        use parse::syntax::Syntax;
        if Self::is_binary(&*rhs.value) {
            let mut lh_op = op;
            let lh_op_precedes;

            {
                let mut rhs_binary = Self::binary_mut(&mut *rhs.value).unwrap();

                lh_op_precedes = Self::precedes(lh_op, rhs_binary.op);
                if lh_op_precedes {
                    // The situation is like X * Y + Z; (X * Y) should go first.
                    //
                    // Currently, however, they're ordered X * (Y + Z), Y and Z.
                    // together in rhs.
                    // So we need to take out Y or Z and put it in lhs.
                    //
                    // We do this with a tortured sequence of swaps for two
                    // reasons:
                    //
                    // 1) We can't have the Syntax trait return a std::any::Any
                    // for generics/?Sized reasons. So we're stuck with
                    // mutation; we can't simply swap the Boxes and return.
                    //
                    // 2) Since we're stuck with swaps, we might as well do so
                    // without swapping in temporary Nones; this means we don't
                    // have to have Option<Box> in the Syntax. Instead we swap
                    // in place.
                    //
                    // Note the order is important for non-commutative ops.

                    let x = &mut lhs;
                    let y = &mut rhs_binary.lhs;
                    let z = &mut rhs_binary.rhs;

                    use std::mem::swap;

                    // X * Y + Z -> Y * X + Z
                    swap(x, y);

                    // Y * X + Z -> Z * X + Y
                    // (Note: confusingly, x and z are now holding Y and Z,
                    // because of the above swap.)
                    swap(x, z);

                    // Swap the ops: Z * X + Y -> Z + X * Y
                    swap(&mut rhs_binary.op, &mut lh_op);

                    // Now we just need to swap the order of the Syntax structs
                    // themselves: Z + X * Y -> X * Y + Z. We do so below once
                    // the &mut drops from scope.
                }
            }

            if lh_op_precedes {
                Some(self::Syntax {
                    lhs: rhs,
                    rhs: lhs,
                    op: lh_op,
                })
            } else {
                Some(self::Syntax {
                    lhs,
                    rhs,
                    op,
                })
            }
        }
        else {
            Some(self::Syntax {
                lhs,
                rhs,
                op,
            })
        }
    }

    // These wrap std::any, for simplification.
    // They can't be put in the trait (it'd have to be generic).

    fn binary(syntax: &syntax::Syntax) -> Option<&Self> {
        use parse::syntax::Syntax;
        use std::any::Any;

        syntax.any()?.downcast_ref::<Self>()
    }

    fn binary_mut(syntax: &mut syntax::Syntax) -> Option<&mut Self> {
        use parse::syntax::Syntax;
        use std::any::Any;

        syntax.any_mut()?.downcast_mut::<Self>()
    }

    fn is_binary(syntax: &syntax::Syntax) -> bool {
        use parse::syntax::Syntax;
        use std::any::Any;

        match syntax.any() {
            Some(any) => any.downcast_ref::<Self>().is_some(),
            None => false,
        }
    }

    // @OPTION move into TokenKind, or a binary op token specific mod

    /// Does `a` precede `b`?
    fn precedes(a: TokenKind, b: TokenKind) -> bool {
        Self::get_precedence(a) > Self::get_precedence(b)
    }

    fn get_precedence(op_kind: TokenKind) -> usize { PRECEDENCE[op_kind] }
}

impl syntax::Syntax for Syntax {
    fn any(&self) -> Option<&std::any::Any> { Some(self) }
    fn any_mut(&mut self) -> Option<&mut std::any::Any> { Some(self) }
}
