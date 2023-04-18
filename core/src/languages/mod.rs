use suzaku_extension_sdk::{
    language::{
        parser::{LanguageParserPolicy, LanguageParsePolicyInfo}, 
        data_cleaner::LanguageDataCleanPolicy,
        analyzer::LanguageAnalysisPolicy, mapper::LanguageMapPolicy,
    }
};
use suzaku_language_extensions::{
    java::{
        parser_policy::{JavaParserPolicy, JavaParserPolicyInfo},
        data_clean_policy::JavaDataCleanPolicy,
        analyzer_policy::JavaAnalyzer, mapper_policy::JavaMapperPolicy,
    },
};

pub struct ExtensionFactory {}

impl ExtensionFactory {
    pub fn get_parser_policy(_language: &str) -> Option<impl LanguageParserPolicy> {
        Some(JavaParserPolicy::new())
    }

    pub fn get_parse_policy_info(_language: &str) -> Option<impl LanguageParsePolicyInfo> {
        Some(JavaParserPolicyInfo{})
    }

    pub fn get_data_clean_policy(language: &str) -> Option<impl LanguageDataCleanPolicy> {
        Some(JavaDataCleanPolicy::new())
    }

    pub fn get_data_mapping_policy(language: &str) -> Option<impl LanguageMapPolicy> {
        Some(JavaMapperPolicy::new())
    }

    pub fn get_analysis_policy(language: &str) -> Option<impl LanguageAnalysisPolicy> {
        Some(JavaAnalyzer::new())
    }
}
