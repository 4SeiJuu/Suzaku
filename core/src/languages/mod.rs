use suzaku_extension_sdk::{
    language::{
        parser::LanguageParserPolicy, 
        analyzer::LanguageAnalysisPolicy
    }
};
use suzaku_language_extensions::{
    java::{
        parser_policy::JavaParserPolicy,
        analyzer_policy::JavaAnalysisPolicy
    },
};

pub struct ExtensionFactory {}

impl ExtensionFactory {
    pub fn get_parser_policy(language: &str) -> Option<impl LanguageParserPolicy> {
        Some(JavaParserPolicy::new())
    }

    pub fn get_analysis_policy(language: &str) -> Option<impl LanguageAnalysisPolicy> {
        Some(JavaAnalysisPolicy::new())
    }
}
