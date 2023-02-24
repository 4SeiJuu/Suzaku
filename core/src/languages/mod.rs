mod java;
mod analyzer;
mod inode;
mod stack;

pub use analyzer::{Analyzer, AnalyzerError, Result};
use java::JavaAnalyzer;

pub struct AnalyzerFactory {}

impl AnalyzerFactory {
    pub fn get_analyzer() -> Option<impl Analyzer> {
        Some(JavaAnalyzer::new())
    }
}