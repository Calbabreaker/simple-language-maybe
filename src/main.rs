fn main() {
    let mut args = std::env::args();
    let file = args.nth(1).unwrap_or("".into());
    match slm::Source::from_file(&file) {
        Ok(source) => {
            let mut runner = slm::Runner::default();
            runner.run(&source);
        }
        Err(err) => eprintln!("{err}"),
    }
}
