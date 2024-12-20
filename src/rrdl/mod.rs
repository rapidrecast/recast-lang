//! RapidRecast Definition Language is the custom language used to configure RapidRecast.

#[cfg(test)]
mod test;

use logos::Logos;

/// The Lexer Tokens available for the RapidRecast Definition Language.
#[derive(Logos)]
pub enum LexerToken {
    /// A segment is a name of a part of file
    #[regex(r"[a-zA-Z]:")]
    Segment,
}

/// Parse RapidRecast Definition Language into a list of LexerTokens.
pub fn parse(input: &str) -> Result<Vec<LexerToken>, ()> {
    let lexer = LexerToken::lexer(input);
    let mut tokens = Vec::new();
    for token in lexer {
        tokens.push(token?);
    }
    Ok(tokens)
}
