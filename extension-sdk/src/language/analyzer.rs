use std::path::PathBuf;

#[derive(Debug)]
pub struct LanguageAnalysisPolicyError {}
pub type LanguageAnalysisResult<T> = std::result::Result<T, LanguageAnalysisPolicyError>;

pub trait LanguageAnalysisPolicy {
    fn new() -> Self;
    fn execute(&mut self, data: &Vec<PathBuf>, output: &PathBuf) -> LanguageAnalysisResult<PathBuf>;
}