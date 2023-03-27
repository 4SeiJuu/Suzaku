use std::path::PathBuf;

#[derive(Debug)]
pub struct LanguageDataCleanPolicyError {}
pub type LanguageDataCleanResult<T> = std::result::Result<T, LanguageDataCleanPolicyError>;

pub trait LanguageDataCleanPolicy {
    fn new() -> Self;
    fn execute(&mut self, metadata: &PathBuf, output: &PathBuf) -> LanguageDataCleanResult<PathBuf>;
}