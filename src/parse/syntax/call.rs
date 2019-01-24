use lex::{ Token, TokenKind };
use parse::{ syntax, TokenIter };

#[derive (Debug)]
pub struct Syntax {
    name: syntax::name::Syntax,
    arguments: Vec<Box<syntax::Syntax>>,
}

impl Syntax {
    pub fn parse<'a, 'b: 'a, I>(
        tokens: &mut TokenIter<'a, 'b, I>,
        name: syntax::name::Syntax,
    ) -> Option<Self> where I: Iterator<Item=&'a Token<'b>> {
        tokens.eat(TokenKind::ParenOpen)?;

        let mut arguments = Vec::new();

        if tokens.peek()?.kind != TokenKind::ParenClose {
            loop {
                arguments.push(Box::new(
                    syntax::name::Syntax::parse(tokens)?
                ) as Box<syntax::Syntax>);

                // Arguments must be delimited by commas
                match tokens.peek()?.kind {
                    TokenKind::GramComma => {
                        // Note I am accepting a ) directly after a , like Rust.
                        tokens.eat(TokenKind::GramComma)?;

                        if tokens.peek()?.kind == TokenKind::ParenClose {
                            break;
                        }
                    },
                    TokenKind::ParenClose => break,
                    _ => return None,
                }
            }
        }
        tokens.eat(TokenKind::ParenClose)?;

        Some(Syntax {
            name,
            arguments,
        })
    }
}

impl syntax::Syntax for Syntax {
    fn any(&self) -> Option<&std::any::Any> { Some(self) }
    fn any_mut(&mut self) -> Option<&mut std::any::Any> { Some(self) }
}
