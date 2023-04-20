use std::path::PathBuf;

#[derive(Debug)]
pub struct LanguageDataExtractorPolicyError {}
pub type LanguageDataExtractorResult<T> = std::result::Result<T, LanguageDataExtractorPolicyError>;

pub trait LanguageDataExtractorPolicy {
    fn new() -> Self;
    fn execute(&mut self, metadata: &PathBuf, output: &PathBuf) -> LanguageDataExtractorResult<PathBuf>;
}