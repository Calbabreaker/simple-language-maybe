use crate::{Lexer, Source};

#[derive(Default)]
pub struct Runner {}

impl Runner {
    /// Run the source code neatly printing an error when there is one
    pub fn run(&mut self, source: Source) {
        if let Err(error) = self.try_run(&source.code) {
            println!("{0}", error.with_source(source));
        }
    }

    /// Trying running the source code returning a result type with the error
    pub fn try_run(&mut self, source: &String) -> crate::Result<()> {
        let lexer = Lexer::parse(source)?;
        println!("{lexer:?}");
        Ok(())
    }
}
