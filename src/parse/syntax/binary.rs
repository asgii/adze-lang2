extern crate enum_map;

use lex::{ Token, TokenKind };
use parse::{ syntax, syntax::SyntaxKind, TokenIter };

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
        // X, Y, Z are the operands in L-R order.
        // lh_op and rh_op are the operators in L-R order.
        //
        // @OPTION clarify by using less mutation, e.g. swap(), which
        // invalidates these names
        //
        let mut rhs = Box::new(syntax::expression::Syntax::parse(tokens)?);
        use parse::syntax::Syntax;
        match rhs.value.kind() {
            SyntaxKind::Binary => {
                let mut lh_op = op;
                let mut lh_op_precedes = false;

                // Use std::any to downcast, for when rhs is another binary::
                // Syntax.
                //
                // Note any() here must return Some. The circumlocution here is
                // so that any, the reference, goes out of scope before we start
                // moving rhs around.
                // (The any must be a &mut, because Any is ?Sized, so you can't
                // have an Option<Any>...)
                //
                if let Some(mut rhs_any) = rhs.value.any() {
                    lh_op_precedes = match rhs_any.downcast_ref::<self::Syntax>(
                    ) {
                        Some(rhs_bin) => Self::precedes(lh_op, rhs_bin.op),
                        _ => panic!(),
                    };

                    if lh_op_precedes {
                        // X * Y + Z
                        // Currently Y and Z are together in rhs.
                        // So we need to take out Y or Z and put it in lhs.
                        //
                        // Note we must take out Z and replace with X; we can't
                        // take out Y, because we don't have what is replacing
                        // it until after it's taken out. This avoids having to
                        // use Options in binary::Syntax.

                        if let Some(
                            mut rhs_bin
                        ) = rhs_any.downcast_mut::<self::Syntax>() {
                            use std::mem::swap;
                            swap(&mut rhs_bin.rhs, &mut lhs);

                            // Now any, i.e. rhs, is Y + X. Swap the ops too.
                            swap(&mut rhs_bin.op, &mut lh_op);
                        }

                        // Now rhs is Y * X.
                        //
                        // @TODO we must rearrange, because what if instead of
                        // *, it's a non-commutative op!
                        // Can do so by swapping perversely swapping more:
                        // Y * X -> Z * X -> Z * Y -> X * Y...!
                        // Would the compiler figure it out?
                    }
                }
                else { panic!() }

                if lh_op_precedes {
                    // Note the order here is not arbitrary, because of the
                    // non-commutativity of some ops.
                    Some(self::Syntax {
                        lhs,
                        rhs,
                        op: lh_op,
                    })
                }
                else {
                    Some(self::Syntax {
                        lhs,
                        rhs,
                        op,
                    })
                }
            },

            _ => Some(self::Syntax {
                lhs,
                rhs,
                op,
            })
        }
    }

    // @OPTION move into TokenKind, or a binary op token specific mod

    /// Does `a` precede `b`?
    fn precedes(a: TokenKind, b: TokenKind) -> bool {
        Self::get_precedence(a) > Self::get_precedence(b)
    }

    fn get_precedence(op_kind: TokenKind) -> usize {
        PRECEDENCE[op_kind]
    }
}

impl syntax::Syntax for Syntax {
    fn kind(&self) -> SyntaxKind { SyntaxKind::Binary }
    fn any(&mut self) -> Option<&mut std::any::Any> { Some(self) }
}
