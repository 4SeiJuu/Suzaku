use suzaku_extension_sdk::{
    language::parser::{
        LanguageParserPolicy, 
    }
};
use suzaku_language_extensions::{
    java::JavaParserPolicy,
};

pub struct AnalyzerFactory {}

impl AnalyzerFactory {
    pub fn get_analyzer(language: &str) -> Option<impl LanguageParserPolicy> {
        Some(JavaParserPolicy::new())
    }
}
