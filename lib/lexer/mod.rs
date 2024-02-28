mod tokens;
pub use tokens::Token;

#[allow(unused, clippy::module_inception)]
mod lexer;

#[cfg(test)]
mod test;

pub use lexer::Lexer;
