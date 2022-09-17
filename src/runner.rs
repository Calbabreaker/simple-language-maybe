use crate::{Lexer, Parser};

#[derive(Debug)]
pub struct Source {
    pub code: String,
    pub file: String,
}

impl Source {
    pub fn from_file(file: &String) -> std::io::Result<Self> {
        let path = std::fs::canonicalize(file)?.to_string_lossy().into();
        let code = std::fs::read_to_string(&path)?;
        Ok(Source { code, file: path })
    }
}

#[derive(Default)]
pub struct Runner {}

impl Runner {
    /// Run the source code printing the error when there is one
    pub fn run(&mut self, source: &Source) {
        if let Err(error) = self.try_run(source) {
            eprintln!("{0}", error.with_source(source));
        }
    }

    /// Trying running the source returning a result type with the error when there is one
    pub fn try_run(&mut self, source: &Source) -> crate::Result<()> {
        let lexer = Lexer::parse(&source.code)?;
        println!("Tokens: {:?}\n", lexer.tokens);
        let parser = Parser::parse(&lexer)?;
        println!("{:?}", parser.tree);
        Ok(())
    }
}
