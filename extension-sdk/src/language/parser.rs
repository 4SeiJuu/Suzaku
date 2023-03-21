use std::path::PathBuf;

#[derive(Debug)]
pub struct LanguageParserPolicyError {}
pub type LanguageParseResult<T> = std::result::Result<T, LanguageParserPolicyError>;

pub trait LanguageParserPolicy {
    fn new() -> Self;
    fn execute(&self, src: &PathBuf, output: &PathBuf) -> LanguageParseResult<PathBuf>;
    fn get_filename_extensions(&self) -> Option<Vec<String>>;
}
