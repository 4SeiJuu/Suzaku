mod java;
mod analyzer;
mod inode;
mod stack;

pub use analyzer::{Analyzer, AnalyzerError, Result};
use java::JavaAnalyzer;

pub const PARSED_RESULT_FOLDER_NAME: &str = "metadata";
pub const ANALYZED_RESULTS_FOLDER_NAME: &str = "analyzed";

pub struct AnalyzerFactory {}

impl AnalyzerFactory {
    pub fn get_analyzer() -> Option<impl Analyzer> {
        Some(JavaAnalyzer::new())
    }
}