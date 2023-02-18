pub trait Analyzer {
    fn new() -> Self;
    fn run(&self, src: &str);
}