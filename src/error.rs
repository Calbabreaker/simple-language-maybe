use crate::Source;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub line: usize,
    pub columns: (usize, usize),
}

impl Position {
    pub fn new(line_num: usize, column: (usize, usize)) -> Self {
        Self {
            line: line_num,
            columns: column,
        }
    }
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ErrorKind {
    #[error("SyntaxError: Invalid token")]
    InvalidToken,
    #[error("SyntaxError: Unmatched {0}")]
    Unmatched(&'static str),
    #[error("NameError: '{0}' is not defined")]
    NameError(String),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub position: Position,
}

impl Error {
    pub fn new(kind: ErrorKind, position: Position) -> Self {
        Self { kind, position }
    }

    pub fn with_source<'a>(&'a self, source: &'a Source) -> ErrorWithSource {
        ErrorWithSource {
            error: self,
            source,
        }
    }
}

pub struct ErrorWithSource<'a> {
    pub error: &'a Error,
    pub source: &'a Source,
}

impl<'a> std::fmt::Display for ErrorWithSource<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{0}", self.error.kind)?;

        let position = self.error.position;
        let (start_col, end_col) = position.columns;
        writeln!(f, "{0}:{1}:{2}", self.source.file, position.line, start_col)?;

        // Print the code snippet
        let mut lines = self.source.code.lines();
        let line_code = lines.nth(position.line).ok_or(std::fmt::Error)?;
        writeln!(f, "{0}", line_code)?;

        // Show where the error is in the snippet
        write!(
            f,
            "{0: <1$}{0:^<2$}",
            "",
            start_col,
            end_col - start_col + 1
        )
    }
}
