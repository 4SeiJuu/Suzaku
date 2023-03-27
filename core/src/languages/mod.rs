use suzaku_extension_sdk::{
    language::{
        parser::LanguageParserPolicy, 
        data_cleaner::LanguageDataCleanPolicy
    }
};
use suzaku_language_extensions::{
    java::{
        parser_policy::JavaParserPolicy,
        data_clean_policy::JavaDataCleanPolicy
    },
};

pub struct ExtensionFactory {}

impl ExtensionFactory {
    pub fn get_parser_policy(language: &str) -> Option<impl LanguageParserPolicy> {
        Some(JavaParserPolicy::new())
    }

    pub fn get_analysis_policy(language: &str) -> Option<impl LanguageDataCleanPolicy> {
        Some(JavaDataCleanPolicy::new())
    }
}
