mod tokens;
pub use tokens::Token;
pub use tokens::TokenDebugInfo;
pub use tokens::TokenType;

#[allow(clippy::module_inception)]
mod lexer;

#[cfg(test)]
mod test;

pub use lexer::Lexer;
pub use lexer::LexerError;
