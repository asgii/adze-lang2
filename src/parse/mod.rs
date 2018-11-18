use lex;

mod syntax {
    pub mod function;
    pub mod signature;
    pub mod block;
    pub mod statement;
    pub mod init;
    pub mod name;
    pub mod literal;

    /// A node in the abstract syntax tree.
    ///
    /// Some `Syntax`es will need generic `dyn Syntax`es because different kinds
    /// of syntax could stand in at that point in the tree.
    /// Where that is not true, it should be ok to type the `Syntax` directly,
    /// without the need of the trait.
    ///
    pub trait Syntax: std::fmt::Debug {
        // parse() doesn't go here. It requires type information.
    }
}

#[derive (Debug)]
pub struct Tree {
    // @TODO dedicated top-level expression
    pub function: syntax::function::Syntax,
}

pub struct Parser {}

impl Parser {

    pub fn new() -> Self {
        Self {}
    }

    pub fn parse<'a>(
        &self,
        tokens: Vec<lex::Token<'a>>
    ) -> Option<Tree> {
        Some(Tree {
            function: syntax::function::Syntax::parse(
                &mut TokenIter::new(tokens.iter())
            )?,
        })
    }
}

// @OPTION move to lex, have as return of lex()
pub struct TokenIter<'a, 'b: 'a, I> where I: Iterator<Item=&'a lex::Token<'b>> {
    tokens: std::iter::Peekable<I>,
}

impl <'a, 'b, I> TokenIter<'a, 'b, I>
where I: Iterator<Item=&'a lex::Token<'b>> {
    pub fn new(tokens: I) -> Self {
        Self {
            tokens: tokens.peekable(),
        }
    }

    /// Expect a `Token` of `TokenKind` `expected`; advance past it if it's
    /// there.
    ///
    pub fn eat(
        &mut self,
        expected: lex::TokenKind,
    ) -> Option<&lex::Token> {
        match self.tokens.peek()? {
            lex::Token { kind, .. } if *kind == expected => (),
            _ => return None,
        }

        self.tokens.next()
    }

    /// Look at the current `Token` without advancing.
    pub fn peek(&mut self) -> Option<&'a lex::Token<'b>> {
        // Peekable::peek() returns a && because we iterate over &.
        Some(*(self.tokens.peek()?))
    }
}
