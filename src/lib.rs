mod error;
mod lexer;
mod parser;
mod runner;

pub use error::{Error, ErrorKind, Position, Result};
pub use lexer::Lexer;
pub use runner::Runner;

#[derive(Debug)]
pub struct Source {
    pub code: String,
    pub file: String,
}

impl Source {
    pub fn from_file(file: String) -> std::io::Result<Self> {
        let code = std::fs::read_to_string(&file)?;
        Ok(Source { code, file })
    }
}
