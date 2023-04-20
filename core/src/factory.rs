use suzaku_extension_sdk::{
    parser::{
        LanguageParserPolicy, 
        LanguageParsePolicyInfo
    }, 
    extractor::LanguageDataExtractorPolicy,
    mapper::LanguageDataMapperPolicy, 
    analyzer::LanguageAnalysisPolicy, 
    reporter::Reporter,
};
use suzaku_language_extensions::{
    java::{
        parser_policy::{
            JavaParserPolicy, 
            JavaParserPolicyInfo
        },
        data_clean_policy::JavaDataCleanPolicy,
        analyzer_policy::JavaAnalyzer,
        mapper_policy::JavaMapperPolicy, 
        graphviz_reporter::GraphvizReporter,
    },
};

pub struct ExtensionFactory {}

impl ExtensionFactory {
    pub fn get_parse_policy_info(_language: &str) -> Option<impl LanguageParsePolicyInfo> {
        Some(JavaParserPolicyInfo{})
    }

    pub fn get_parser_policy(_language: &str) -> Option<impl LanguageParserPolicy> {
        Some(JavaParserPolicy::new())
    }

    pub fn get_data_extractor_policy(language: &str) -> Option<impl LanguageDataExtractorPolicy> {
        Some(JavaDataCleanPolicy::new())
    }

    pub fn get_data_mapper_policy(language: &str) -> Option<impl LanguageDataMapperPolicy> {
        Some(JavaMapperPolicy::new())
    }

    pub fn get_analyzer_policy(language: &str) -> Option<impl LanguageAnalysisPolicy> {
        Some(JavaAnalyzer::new())
    }

    pub fn get_reporter(reportor_name: &str) -> Option<impl Reporter> {
        Some(GraphvizReporter::new())
    }
}
