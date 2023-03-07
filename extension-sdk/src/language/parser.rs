use std::path::PathBuf;

pub struct LanguageParserPolicyError {}
pub type Result<String> = std::result::Result<String, LanguageParserPolicyError>;

pub trait LanguageParserPolicy {
    fn new() -> Self;
    fn execute(&self, src: &PathBuf, output_dir: &PathBuf) -> Result<String>;
    fn get_filename_extension(&self) -> &str;
}
