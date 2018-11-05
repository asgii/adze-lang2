use std::collections::HashMap;

// @TODO move out of lex
//
#[derive (Debug, Copy, Clone)]
pub enum TokenKind {
    // 'Grammar'
    GramComma,
    GramSemicolon,
    GramColon,
    GramComment,

    BraceOpen,
    BraceClose,
    ParenOpen,
    ParenClose,

    // 'Operation'
    OpAssign,
    OpAdd,
    OpSub,

    // 'Other': placeholder name
    OthName,
    OthInvalid,
}

// @TODO better way of doing this
//
fn parse_map() -> HashMap<&'static str, TokenKind> {

    let mut parse_map = HashMap::new();

    parse_map.insert(",", TokenKind::GramComma);
    parse_map.insert(";", TokenKind::GramSemicolon);
    parse_map.insert(":", TokenKind::GramColon);
    parse_map.insert("#", TokenKind::GramComment);

    parse_map.insert("{", TokenKind::BraceOpen);
    parse_map.insert("}", TokenKind::BraceClose);
    parse_map.insert("(", TokenKind::ParenOpen);
    parse_map.insert(")", TokenKind::ParenClose);

    parse_map.insert("=", TokenKind::OpAssign);
    parse_map.insert("+", TokenKind::OpAdd);
    parse_map.insert("-", TokenKind::OpSub);

    parse_map
}

pub struct Token<'a> {
    kind: TokenKind,
    source: &'a str,
}

impl<'a> Token<'a> {

    pub fn new(
        kind: TokenKind,
        source: &'a str,
    ) -> Self {
        Token {
            kind,
            source,
        }
    }
}

pub struct Lexer {
    parse_map: HashMap<&'static str, TokenKind>,
}

impl Lexer {

    pub fn new() -> Self {
        Self {
            parse_map: parse_map(),
        }
    }

    pub fn lex<'a>(
        &self,
        source: &'a str,
    ) -> Vec<Token<'a>> {
        let mut tokens = Vec::new();

        // Iterators are lazy
        //
        // @OPTION however, this will still probably search for a \n every time
        // rather than simply stopping when it hits one. Custom iterators?
        //
        let lines = source.lines();
        for line in lines {
            let words = line.split_whitespace();

            for word in words {
                tokens.push(self.lex_word(word));
            }
        }

        tokens
    }

    fn lex_word<'a>(
        &self,
        word: &'a str
    ) -> Token<'a> {
        match self.parse_map.get(word) {
            Some(kind) => Token::new(*kind, word),
            None => {
                // @TODO check name, otherwise invalid
                Token::new(TokenKind::OthInvalid, word)
            },
        }
    }
}
