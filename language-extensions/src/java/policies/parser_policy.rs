use std::{
    fs,
    path::PathBuf,
};

use antlr_rust::{
    InputStream,
    common_token_stream::CommonTokenStream,
};

use suzaku_extension_sdk::{
    ATTR_FOLDER,
    ATTR_FILE,
    SRC_FILE_EXTENSION,
    parser::{
        LanguageParserPolicy, 
        LanguageParsePolicyInfo,
        LanguageParserPolicyError,
        LanguageParserListener,
        LanguageParseResult,
    },
    meta::{
        IMetadata,
        Metadata
    },
    reorganzier::LanguageMetaReorganizePolicy, 
    parser_listener::ParserListener,
};

use crate::java::data::meta_type::MetaType;

use super::{
    super::generated::{
        javalexer::JavaLexer,
        javaparser::*,
    },
    reorganizer::JavaMetaReorganizePolicy
};


#[derive(Debug, Clone, Copy)]
pub struct JavaParserPolicy;

impl JavaParserPolicy {
    fn parse(&self, folder: &PathBuf, relative_file_path: &PathBuf) -> Option<Metadata<MetaType>> {
        let src = PathBuf::from_iter(vec![folder, relative_file_path]);
        if src.is_file() && src.exists() {
            let content = fs::read_to_string(src).expect("should read context of file");
            let data = InputStream::new(content.trim());
    
            let lexer = JavaLexer::new(data);
            let token_source = CommonTokenStream::new(lexer);
    
            let mut file_node = Metadata::new(MetaType::File);
            file_node.set_attr(ATTR_FOLDER, folder.to_str().unwrap());
            file_node.set_attr(ATTR_FILE, relative_file_path.to_str().unwrap());

            let reorganizer = JavaMetaReorganizePolicy::new();
            let parser_listener: ParserListener<MetaType> = ParserListener::new(file_node, Some(Box::new(reorganizer)));
    
            let mut parser = JavaParser::new(token_source);
            let listener_id = parser.add_parse_listener(Box::new(parser_listener));
            parser.compilationUnit();

            let mut parser_listener = parser.remove_parse_listener(listener_id);
            return parser_listener.results();
        }
        None
    }
}

impl<'consumer> LanguageParserPolicy for JavaParserPolicy {
    fn new() -> Self {
        JavaParserPolicy {}
    }

    fn execute(&self, folder: &PathBuf, relative_file_path: &PathBuf) -> LanguageParseResult<String> {
        if let Some(tree) = self.parse(folder, relative_file_path) {
            return Ok(tree.dump().unwrap());
        }
        Err(LanguageParserPolicyError {})
    }
}

pub struct JavaParserPolicyInfo;
impl LanguageParsePolicyInfo for JavaParserPolicyInfo {
    fn get_filename_extensions(&self) -> Option<Vec<String>> {
        Some(vec![String::from(SRC_FILE_EXTENSION)])
    }
}
