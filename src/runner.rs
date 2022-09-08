use crate::{Lexer, Source};

#[derive(Default)]
pub struct Runner {}

impl Runner {
    /// Run the source code printing the error when there is one
    pub fn run(&mut self, source: &Source) {
        if let Err(error) = self.try_run(source) {
            println!("{0}", error.with_source(source));
        }
    }

    /// Trying running the source returning a result type with the error when there is one
    pub fn try_run(&mut self, source: &Source) -> crate::Result<()> {
        let lexer = Lexer::parse(&source.code)?;
        println!("{lexer:?}");
        Ok(())
    }
}
