use std::path::PathBuf;

pub struct AnalyzerError {}
pub type Result<String> = std::result::Result<String, AnalyzerError>;

pub trait Analyzer {
    fn new() -> Self;
    fn execute(&self, src: &PathBuf, output_dir: &PathBuf) -> Result<String>;
    fn get_src_file_extension(&self) -> &str;
}