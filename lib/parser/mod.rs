#[allow(clippy::module_inception)]
mod parser;

pub use parser::Parser;

#[cfg(test)]
mod test;

mod ast;
