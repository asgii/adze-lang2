extern crate phf;

#[derive (Debug, Copy, Clone, PartialEq)]
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
    KeyIf, // @OPTION do I want if/else?
    KeyElse,

    // 'Literal'
    LitInteger,

    // 'Other': placeholder name
    OthName,
    OthInvalid,
}

/// Whether a token can validly follow immediately after the last one without
/// a line-ending or whitespace.
///
/// e.g., `return;` is two valid tokens, because `;` can follow immediately.
///
enum TokenCanFollowImmediately {
    Can,
    Cannot,
}

struct TokenProperties {
    pub kind: TokenKind,
    pub can_follow: TokenCanFollowImmediately,
}

static SYMBOLS: phf::Map<&'static str, TokenProperties> = phf_map! {
    "," => TokenProperties {
        kind: TokenKind::GramComma,
        can_follow: TokenCanFollowImmediately::Can
    },
    ";" => TokenProperties {
        kind: TokenKind::GramSemicolon,
        can_follow: TokenCanFollowImmediately::Can,
    },
    ":" => TokenProperties {
        kind: TokenKind::GramColon,
        can_follow: TokenCanFollowImmediately::Can,
    },
    "#" => TokenProperties {
        kind: TokenKind::GramComment,
        can_follow: TokenCanFollowImmediately::Can,
    },

    "=" => TokenProperties {
        kind: TokenKind::OpAssign,
        can_follow: TokenCanFollowImmediately::Cannot,
    },
    "+" => TokenProperties {
        kind: TokenKind::OpAdd,
        can_follow: TokenCanFollowImmediately::Cannot,
    },
    "-" => TokenProperties {
        kind: TokenKind::OpSub,
        can_follow: TokenCanFollowImmediately::Cannot,
    },

    "{" => TokenProperties {
        kind: TokenKind::BraceOpen,
        can_follow: TokenCanFollowImmediately::Cannot,
    },
    "}" => TokenProperties {
        kind: TokenKind::BraceClose,
        can_follow: TokenCanFollowImmediately::Cannot,
    },
    "(" => TokenProperties {
        kind: TokenKind::ParenOpen,
        can_follow: TokenCanFollowImmediately::Can,
    },
    ")" => TokenProperties {
        kind: TokenKind::ParenClose,
        can_follow: TokenCanFollowImmediately::Can,
    },
};

/// Key-words are kept separately from symbols because the former must be
/// delimited from other tokens, where the latter have exceptions.
///
static KEYWORDS: phf::Map<&'static str, TokenKind> = phf_map! {
    "return" => TokenKind::KeyReturn,
    "if"     => TokenKind::KeyIf,
    "else"   => TokenKind::KeyElse,
};

#[derive (Debug)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub source: &'a str,
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

enum WhitespaceState {
    StartOfLine,
    Whitespace,
    NoWhitespace,
    EndOfLine,
}

struct TokenIter<'a> {
    word: &'a str,

    /// Whether there was whitespace (or the start or end to a line) immediately
    /// before the start of `word`.
    whitespace: WhitespaceState,
}

impl <'a> TokenIter<'a> {
    pub fn new(word: &'a str) -> TokenIter<'a> {
        TokenIter {
            word,
            whitespace: WhitespaceState::StartOfLine,
        }
    }

    /// Advance past any whitespace; return whether there was any whitespace.
    ///
    fn eat_whitespace(&mut self) {
        let was_whitespace = match self.word.bytes().next() {
            None => WhitespaceState::EndOfLine,
            Some(first_char) => match first_char.is_ascii_whitespace() {
                true  => WhitespaceState::Whitespace,
                false => WhitespaceState::NoWhitespace,
            },
        };

        let mut len_whitespace = 0;
        for this_char in self.word.bytes() {
            if this_char.is_ascii_whitespace() {
                len_whitespace += 1;
            }
            else {
                break;
            }
        }
        self.word = &self.word[len_whitespace..];

        // To have this function encapsulate whitespace-eating completely, it
        // must handle the start of the line. We want to eat whitespace at the
        // start of the line, but unlike other cases, we don't want to return
        // whether there was whitespace, since, for the purposes of symbol
        // validity, the start of the line should count as whitespace!
        //
        // So we treat the start specially, ignoring any lack of whitespace when
        // setting `whitespace`.
        //
        if let WhitespaceState::StartOfLine = self.whitespace {
            self.whitespace = WhitespaceState::Whitespace;
        }
        else {
            self.whitespace = was_whitespace;
        }
    }

    /// Advance until the next symbol or whitespace/line-ending; return either
    /// a valid or invalid name token.
    ///
    fn eat_name(&mut self) -> Token<'a> {
        assert_ne!(self.word.len(), 0);

        let mut valid = true;

        let mut remaining = self.word;
        // Note len() is number of bytes, not chars. We'll stick to bytes (we
        // are enforcing ASCII anyway).
        while remaining.len() > 0 {
            if remaining.bytes().next().unwrap().is_ascii_whitespace() {
                break;
            }

            // Don't just break on invalidity; break on either whitespace or an
            // in principle valid symbol.
            // This makes the return the most useful, since it groups together
            // invalid characters as one Token, rather than acting naively as if
            // they were completely unrelated invalidities.

            let mut found_symbol = None;
            let mut len_max_symbol = 0;
            for symbol in SYMBOLS.keys() {
                if symbol.len() > self.word.len() {
                    continue;
                }
                let trunc = &remaining[..symbol.len()];

                if symbol.len() > len_max_symbol {
                    len_max_symbol = symbol.len();
                }

                if *symbol == trunc {
                    found_symbol = Some(*symbol);
                    break;
                }
            }

            match found_symbol {
                None => {
                    // len_max_symbol is the length we've checked against
                    // symbols; we can advance that much.
                    remaining = &remaining[len_max_symbol..];
                },
                Some(_) => {
                    // Don't change remaining; it correctly starts just after
                    // the end of the name.
                    break;
                },
            }
        }

        // @OPTION is there a better way to compare slices?
        //
        let len_name = unsafe {
            remaining.as_ptr().offset_from(self.word.as_ptr())
        };
        let len_name = len_name as usize;

        let name = &self.word[..len_name];

        self.word = &self.word[len_name..];

        let kind = lex_name(name);

        Token::new(kind, name)
    }
}

impl <'a> Iterator for TokenIter<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // Previous iterations will have advanced us just past the end of the
        // last word (or line).
        //
        self.eat_whitespace();

        // The only case where we stop iterating is when we run out of word.
        //
        if self.word.len() == 0 {
            return None;
        }

        // Try to match a symbol or key-word.

        for (symbol, TokenProperties {
            kind, can_follow,
        }) in SYMBOLS.entries() {
            if symbol.len() > self.word.len() {
                continue;
            }
            // Note Range for str is in bytes, not chars (as is len()).
            let trunc = &self.word[..symbol.len()];

            if *symbol == trunc {
                self.word = &self.word[symbol.len()..];

                return match self.whitespace {
                    WhitespaceState::NoWhitespace => {
                        // Some symbols are allowed without whitespace
                        // immediately before them; we have to do this
                        // additional check.
                        //
                        match can_follow {
                            TokenCanFollowImmediately::Can => Some(Token::new(*kind, trunc)),
                            TokenCanFollowImmediately::Cannot => Some(Token::new(TokenKind::OthInvalid, trunc)),
                        }
                    },
                    _ => Some(Token::new(*kind, trunc)),
                };
            }
        }

        for (keyword, token) in KEYWORDS.entries() {
            if keyword.len() > self.word.len() {
                continue;
            }
            let trunc = &self.word[..keyword.len()];

            if *keyword == trunc {
                // This could still be a name, e.g. main_a.
                // We must check what comes afterward.
                //
                let after_keyword = &self.word[keyword.len()..];
                if let Some(next_char) = after_keyword.bytes().next() {
                    // We have to check more than just whitespace, because of
                    // cases like main().
                    // Effectively we're checking for in principle validity of a
                    // character within a name - not actual validity, which will
                    // be checked by eat_name().
                    //
                    if next_char == '_' as u8 ||
                       next_char.is_ascii_alphanumeric() {
                        // continue, not break, in case some symbols contain
                        // others, as e.g. += contains +.
                        continue;
                    }
                }

                self.word = &self.word[keyword.len()..];

                // Key-words must be preceded by whitespace; this determines
                // validity as a key-word.
                //
                // @OPTION technically this is backwards; we could have checked
                // higher up whether there was whitespace, and if not, only
                // matched against symbols.
                //
                return match self.whitespace {
                    WhitespaceState::NoWhitespace => Some(Token::new(TokenKind::OthInvalid, trunc)),
                    _ => Some(Token::new(*token, trunc)),
                };
            }
        }

        // We haven't matched a symbol or key-word.
        // We have to pass down a name, assuming it's valid.
        //
        Some(self.eat_name())
    }
}

/// The state of lexing with regard to comments.
///
#[derive (Debug, Copy, Clone)]
enum LexerState {
    Lexing,
    MidComment,
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

            let mut iter = TokenIter::new(line);

            while let Some(token) = iter.next() {
                let (new_token, new_state) = Self::handle_comments(token, state);

                state = new_state;

                if let Some(some_token) = new_token {
                    tokens.push(some_token);
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

fn lex_name(source: &str) -> TokenKind {
    if is_valid_integer(source) {
        return TokenKind::LitInteger;
    }
    else if is_valid_name(source) {
        return TokenKind::OthName;
    }
    TokenKind::OthInvalid
}

fn is_valid_name(mut source: &str) -> bool {
    assert_ne!(source.len(), 0);

    // Exceptions for first char
    // @OPTION as in Rust, accept leading underscore
    let first_char = source.bytes().next().unwrap();
    if !first_char.is_ascii_alphabetic() {
        return false;
    }
    let mut prev_char = first_char;
    // Advance so the first char isn't counted under the below
    source = &source[1..];

    for this_char in source.bytes() {
        // Allow _ as exception to alphanumericality
        if this_char == '_' as u8 {
            // Don't allow __
            if prev_char == '_' as u8 {
                return false;
            }
        }

        else if !this_char.is_ascii_alphanumeric() {
            return false;
        }
        prev_char = this_char;
    }

    // Exceptions for last char: don't accept _
    if prev_char == '_' as u8 {
        return false;
    }
    true
}

fn is_valid_integer(source: &str) -> bool {
    match get_integer(source) {
        Some(_) => true,
        None => false,
    }
}

/// Parse an integer.
pub fn get_integer(source: &str) -> Option<i32> {
    // @TODO technically Rust's integer format won't be ok because it includes
    // e.g., 1i32
    match source.parse::<i32>() {
        Ok(int) => Some(int),
        Err(_)  => None,
    }
}