pub struct AnalyzerError {}
pub type Result<String> = std::result::Result<String, AnalyzerError>;

pub trait Analyzer {
    fn new() -> Self;
    fn execute(&self, src: &str, output_dir: &str) -> Result<String>;
}