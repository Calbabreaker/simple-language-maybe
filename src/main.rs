fn main() {
    match get_source() {
        Ok(source) => {
            let mut runner = slm::Runner::default();
            runner.run(&source);
        }
        Err(err) => eprintln!("{err}"),
    }
}

fn get_source() -> Result<slm::Source, String> {
    let mut args = std::env::args();
    let file = args.nth(1).ok_or("Expected first argument to be a file")?;
    slm::Source::from_file(file).map_err(|err| format!("{err}"))
}
