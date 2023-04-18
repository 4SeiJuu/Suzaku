use std::path::PathBuf;

#[derive(Debug)]
pub struct LanguageMapPolicyError {}
pub type LanguageMapResult<T> = std::result::Result<T, LanguageMapPolicyError>;

pub trait LanguageMapPolicy {
    fn new() -> Self;
    fn execute(&mut self, data: &PathBuf, output: &PathBuf) -> LanguageMapResult<PathBuf>;
}