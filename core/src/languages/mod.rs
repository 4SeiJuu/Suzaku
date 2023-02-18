mod java;
mod analyzer;
mod inode;

pub use analyzer::Analyzer;
use java::JavaAnalyzer;

pub struct AnalyzerFactory {}

impl AnalyzerFactory {
    pub fn get_analyzer() -> Option<impl Analyzer> {
        Some(JavaAnalyzer::new())
    }
}