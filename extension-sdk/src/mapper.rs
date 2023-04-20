use std::path::PathBuf;

#[derive(Debug)]
pub struct LanguageDataMapperPolicyError {}
pub type LanguageDataMapperResult<T> = std::result::Result<T, LanguageDataMapperPolicyError>;

pub trait LanguageDataMapperPolicy {
    fn new() -> Self;
    fn execute(&mut self, data: &Vec<PathBuf>, output: &PathBuf) -> LanguageDataMapperResult<PathBuf>;
}