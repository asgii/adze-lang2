use lex::{ Token, TokenKind };
use parse::{ syntax, TokenIter };

#[derive (Debug)]
pub struct Syntax {
    lhs: syntax::name::Syntax,
    lhs_type: Option<syntax::name::Syntax>,
    rhs: Box<syntax::Syntax>,
}

impl Syntax {
    pub fn parse<'a, 'b: 'a, I>(
        tokens: &mut TokenIter<'a, 'b, I>,
    ) -> Option<Self> where I: Iterator<Item=&'a Token<'b>> {
        let lhs = syntax::name::Syntax::parse(tokens)?;

        // a := b
        // a: int = b

        tokens.eat(TokenKind::GramColon);
        // @TODO there should be a space between : and =, only if the name after
        // the : is not a type name. (Likewise no space between a and : if there
        // is.) How could you tell?
        // Surely by the location of the closing ;.
        // Note however the Tokens don't currently have the backing source so we
        // could find out - they only have their slices of it...!
        // Move to lexer, perhaps?
        let lhs_type = match tokens.peek()? {
            Token { kind: TokenKind::OpAssign, .. } => {
                None
            },
            Token { kind: TokenKind::OthName, .. } => {
                Some(syntax::name::Syntax::parse(tokens)?)
            },
            _ => return None,
        };
        tokens.eat(TokenKind::OpAssign)?;

        // @TODO evaluable expressions
        let rhs = match tokens.peek()? {
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
            _ => return None,
        };

        tokens.eat(TokenKind::GramSemicolon);

        Some(Syntax {
            lhs,
            lhs_type,
            rhs,
        })
    }
}

impl syntax::Syntax for Syntax {}
