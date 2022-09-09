fn main() {
    let source = slm::Source {
        code: "hello = 2 #* Hello \n no # no".into(),
        file: "<stdin>".into(),
    };

    let mut runner = slm::Runner::default();
    runner.run(&source);
}
