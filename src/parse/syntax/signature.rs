use lex::{ Token, TokenKind };
use parse::{ syntax, TokenIter };

#[derive (Debug)]
pub struct Syntax {
    name: syntax::name::Syntax,
    parameters: Vec<Box<syntax::Syntax>>,
}

impl Syntax {
    pub fn parse<'a, 'b: 'a, I>(
        tokens: &mut TokenIter<'a, 'b, I>,
    ) -> Option<Self> where I: Iterator<Item=&'a Token<'b>> {
        let name = syntax::name::Syntax::parse(tokens)?;
        tokens.eat(TokenKind::ParenOpen)?;

        // @TODO
        let mut parameters = Vec::new();
        /*
        let parameter_iter = std::iter::repeat_with(|| {
            syntax::parameter::Syntax::parse(&mut tokens)?
        });
        for parameters in parameter_iter {
            parameters.push(paramater);
        }
        */

        tokens.eat(TokenKind::ParenClose)?;

        Some(Syntax {
            name,
            parameters,
        })
    }
}

impl syntax::Syntax for Syntax {
    fn any(&self) -> Option<&std::any::Any> { Some(self) }
    fn any_mut(&mut self) -> Option<&mut std::any::Any> { Some(self) }
}
