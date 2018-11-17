use lex::{ Token, TokenKind };
use parse::{ expr, TokenIter };

#[derive (Debug)]
pub struct Expr {
    statements: Vec<Box<expr::Expr>>,
}

impl Expr {
    pub fn parse<'a, 'b: 'a, I>(
        tokens: &mut TokenIter<'a, 'b, I>,
    ) -> Option<Self> where I: Iterator<Item=&'a Token<'b>> {
        tokens.eat(TokenKind::BraceOpen)?;

        let mut statements = Vec::new();
        while match tokens.peek()? {
            Token { kind: TokenKind::BraceClose, .. } => false,
            _ => true,
        } {
            // Currently statement::Expr handles cases like blocks within blocks
            // etc.
            statements.push(Box::new(
                expr::statement::Expr::parse(tokens)?) as Box<expr::Expr>
            );
        }

        tokens.eat(TokenKind::BraceClose)?;

        Some(Expr {
            statements,
        })
    }
}

impl expr::Expr for Expr {

}
