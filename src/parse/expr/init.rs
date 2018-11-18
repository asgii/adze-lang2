use lex::{ Token, TokenKind };
use parse::{ expr, TokenIter };

#[derive (Debug)]
pub struct Expr {
    lhs: expr::name::Expr,
    lhs_type: Option<expr::name::Expr>,
    rhs: Box<expr::Expr>,
}

impl Expr {
    pub fn parse<'a, 'b: 'a, I>(
        tokens: &mut TokenIter<'a, 'b, I>,
    ) -> Option<Self> where I: Iterator<Item=&'a Token<'b>> {
        let lhs = expr::name::Expr::parse(tokens)?;

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
                Some(expr::name::Expr::parse(tokens)?)
            },
            _ => return None,
        };
        tokens.eat(TokenKind::OpAssign)?;

        // @TODO evaluable expressions
        let rhs = match tokens.peek()? {
            Token { kind: TokenKind::LitInteger, .. } => {
                Box::new(
                    expr::literal::Expr::parse(tokens)?
                ) as Box<expr::Expr>
            },
            Token { kind: TokenKind::OthName, .. } => {
                Box::new(
                    expr::name::Expr::parse(tokens)?
                ) as Box<expr::Expr>
            },
            _ => return None,
        };

        tokens.eat(TokenKind::GramSemicolon);

        Some(Expr {
            lhs,
            lhs_type,
            rhs,
        })
    }
}

impl expr::Expr for Expr {}
