#[allow(clippy::module_inception)]
mod parser;

pub use parser::Parser;

#[allow(unused)]
mod ast;

#[cfg(test)]
mod test;
