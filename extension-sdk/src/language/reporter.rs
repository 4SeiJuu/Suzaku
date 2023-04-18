pub trait Reporter {
    fn new() -> Self;
    fn generate();
}