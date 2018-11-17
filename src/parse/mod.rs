use lex;

mod expr {
    pub mod function;
    pub mod signature;
    pub mod block;
    pub mod statement;

    use lex;

    pub trait Expr: std::fmt::Debug {
        // parse() doesn't go here. It requires type information.
    }

    // @TODO parse() should
    // @OPTION/@TODO parse() should probably return a specific Expr
}

pub struct Tree {
    // @TODO dedicated top-level expression
    pub function: expr::function::Expr,
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
            function: expr::function::Expr::parse(
                &mut tokens.iter()
            )?,
        })
    }
}
