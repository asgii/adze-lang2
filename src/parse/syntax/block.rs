use lex::{ Token, TokenKind };
use parse::{ syntax, TokenIter };

#[derive (Debug)]
pub struct Syntax {
    statements: Vec<Box<syntax::Syntax>>,
}

impl Syntax {
    pub fn parse<'a, 'b: 'a, I>(
        tokens: &mut TokenIter<'a, 'b, I>,
    ) -> Option<Self> where I: Iterator<Item=&'a Token<'b>> {
        tokens.eat(TokenKind::BraceOpen)?;

        // Collect statements until a }.
        let mut statements = Vec::new();
        while match tokens.peek()? {
            Token { kind: TokenKind::BraceClose, .. } => false,
            _ => true,
        } {
            statements.push(Box::new(
                syntax::statement::Syntax::parse(tokens)?) as Box<syntax::Syntax>
            );
        }
        tokens.eat(TokenKind::BraceClose)?;

        Some(Syntax {
            statements,
        })
    }
}

impl syntax::Syntax for Syntax {

}
