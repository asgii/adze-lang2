use lex::{ Token, TokenKind };
use parse::expr;

pub struct Expr {
    signature: Box<expr::Expr>,
    statements: Vec<Box<expr::Expr>>,
}

impl Expr {
    pub fn parse<'a, 'b: 'a, I>(
        tokens: &mut I
    ) -> Option<Self> where I: Iterator<Item=&'a Token<'b>> {
        let signature = expr::signature::Expr::parse(tokens)?;

        // @TODO make a lex::Tokens iterator, have a standardised expect()
        match tokens.next()? {
            Token { kind: TokenKind::BraceOpen, .. } => (),
            _ => return None,
        }

        // TODO these shouldn't just be statements. e.g. nested blocks, etc.
        // @OPTION could be useful to bundle repeat_with()
        let mut statements = Vec::new();
        let statement_iter = std::iter::repeat_with(|| {
            expr::statement::Expr::parse(tokens)
        });
        for statement in statement_iter {
            statements.push(Box::new(statement?) as Box<expr::Expr>);
        }

        Some(Expr {
            signature: Box::new(signature),
            statements,
        })
    }
}

impl expr::Expr for Expr {

}
