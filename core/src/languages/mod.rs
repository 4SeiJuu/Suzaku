use suzaku_extension_sdk::{
    language::parser::{
        LanguageParserPolicy, 
    }
};
use suzaku_language_extensions::{
    java::JavaParserPolicy,
};

pub struct ExtensionFactory {}

impl ExtensionFactory {
    pub fn get_parser_policy(language: &str) -> Option<impl LanguageParserPolicy> {
        Some(JavaParserPolicy::new())
    }
}
