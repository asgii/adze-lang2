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

                if let Some(token) = self.lex_word(word, &mut state) {
                    tokens.push(token);
                }
            }
        }

        tokens
    }

    fn lex_word<'a>(
        &self,
        word: &'a str,
        state: &mut LexerState,
    ) -> Option<Token<'a>> {
        let mut token = Self::match_symbols(word);

        if token.is_none() {
            token = Self::match_keywords(word);
        }

        // @TODO check name, otherwise invalid
        //
        let token = match token {
            Some(tok) => tok,
            None => Token::new(TokenKind::OthInvalid, word),
        };

        let (token, next_state) = Self::handle_comments(token, *state);

        *state = next_state;

        token
    }

    fn match_symbols<'a>(word: &'a str) -> Option<Token<'a>> {
        match SYMBOLS.get(word) {
            Some(kind) => Some(Token::new(*kind, word)),
            None => None,
        }
    }

    fn match_keywords<'a>(word: &'a str) -> Option<Token<'a>> {
        match KEYWORDS.get(word) {
            Some(kind) => Some(Token::new(*kind, word)),
            None => None,
        }
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
