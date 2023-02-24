pub struct AnalyzerError {}
pub type Result<String> = std::result::Result<String, AnalyzerError>;

pub trait Analyzer {
    fn new() -> Self;
    fn analysis(&self, src: &str) -> Result<String>;
}