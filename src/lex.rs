extern crate phf;

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

    // 'Keyword'
    KeyReturn,
    KeyIf,
    KeyElse,

    // 'Other': placeholder name
    OthName,
    OthInvalid,
}

#[derive (Debug, Copy, Clone)]
enum LexerState {
    Lexing,
    MidComment,
}

static SYMBOLS: phf::Map<&'static str, TokenKind> = phf_map! {
    "," => TokenKind::GramComma,
    ";" => TokenKind::GramSemicolon,
    ":" => TokenKind::GramColon,
    "#" => TokenKind::GramComment,

    "=" => TokenKind::OpAssign,
    "+" => TokenKind::OpAdd,
    "-" => TokenKind::OpSub,

    "{" => TokenKind::BraceOpen,
    "}" => TokenKind::BraceClose,
    "(" => TokenKind::ParenOpen,
    ")" => TokenKind::ParenClose,
};

static KEYWORDS: phf::Map<&'static str, TokenKind> = phf_map! {
    "return" => TokenKind::KeyReturn,
    "if"     => TokenKind::KeyIf,
    "else"   => TokenKind::KeyElse,
};

#[derive (Debug)]
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

struct TokenIter<'a> {
    word: &'a str,
}

impl <'a> TokenIter<'a> {
    pub fn new(word: &'a str) -> TokenIter<'a> {
        TokenIter {
            word,
        }
    }
}

impl <'a> Iterator for TokenIter<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // The only case where we stop iterating is when we run out of word.
        //
        if self.word.len() == 0 {
            return None;
        }

        // @TODO make generic fn of this
        //
        for (symbol, token) in SYMBOLS.entries() {
            if symbol.len() > self.word.len() {
                continue;
            }
            let trunc = &self.word[..symbol.len()];

            if *symbol == trunc {
                self.word = &self.word[symbol.len()..];

                return Some(Token::new(*token, trunc));
            }
        }

        for (keyword, token) in KEYWORDS.entries() {
            if keyword.len() > self.word.len() {
                continue;
            }
            let trunc = &self.word[..keyword.len()];

            if *keyword == trunc {
                self.word = &self.word[keyword.len()..];

                return Some(Token::new(*token, trunc));
            }
        }

        // @TODO names

        // @TODO this doesn't handle e.g. 'varName;', because the semicolon is
        // afterward.
        // This can probably be handled with names, because getting a name will
        // involve eating until a non-letter/digit.

        let remaining = self.word;
        self.word = "";
        Some(Token::new(TokenKind::OthInvalid, remaining))
    }
}

pub struct Lexer {

}

impl Lexer {

    pub fn new() -> Self {
        Self {}
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
            let mut state = LexerState::Lexing;

            let words = line.split_whitespace();
            for word in words {
                let mut iter = TokenIter::new(word);

                // @TODO this is not exactly right.

                while let Some(token) = iter.next() {
                    let (new_token, new_state) =
                        Self::handle_comments(token, state);

                    state = new_state;

                    if let Some(some_token) = new_token {
                        tokens.push(some_token);
                    }
                }
            }
        }

        tokens
    }

    /// Decide whether or not to keep `token`, given `state`, the state of
    /// comment lexing.
    ///
    /// Returns the new state along with the presence or absence of `token`.
    ///
    fn handle_comments<'a>(
        token: Token<'a>,
        state: LexerState,
    ) -> (Option<Token<'a>>, LexerState) {
        match (&state, &token.kind) {
            // Start ignoring on comment
            (&LexerState::Lexing, &TokenKind::GramComment) => {
                (None, LexerState::MidComment)
            },

            // Stop ignoring on end of comment
            (&LexerState::MidComment, &TokenKind::GramComment) => {
                (None, LexerState::Lexing)
            },

            // Ignore when mid-comment
            (&LexerState::MidComment, _) => {
                (None, LexerState::MidComment)
            },

            (&LexerState::Lexing, _) => {
                (Some(token), LexerState::Lexing)
            },
        }
    }
}
