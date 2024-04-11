mod tokens;
pub use tokens::Token;
pub use tokens::TokenType;
pub use tokens::TokenDebugInfo;

#[allow(clippy::module_inception)]
mod lexer;

#[cfg(test)]
mod test;

pub use lexer::Lexer;
pub use lexer::LexerError;
