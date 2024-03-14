#[allow(clippy::module_inception)]
mod parser;

pub use parser::parse_from_lexer;
pub use parser::parse_from_string;

#[cfg(test)]
mod test;
