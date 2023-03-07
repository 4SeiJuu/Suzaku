use suzaku_extension_sdk::{
    language::parser::{
        LanguageParserPolicy, 
    }
};
use suzaku_language_extensions::{
    java::JavaParserPolicy,
    java9::Java9ParserPolicy
};

pub struct AnalyzerFactory {}

impl AnalyzerFactory {
    pub fn get_analyzer(language: &str) -> Option<impl LanguageParserPolicy> {
        Some(Java9ParserPolicy::new())
    }
}
