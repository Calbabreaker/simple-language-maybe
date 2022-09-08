mod error;
mod lexer;
mod runner;

pub use error::{Error, ErrorKind, Position, Result};
pub use lexer::Lexer;
pub use runner::Runner;

#[derive(Debug)]
pub struct Source {
    pub code: String,
    pub file: String,
}
