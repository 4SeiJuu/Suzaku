use std::{
    fs,
    fs::File,
    io::Write,
    path::PathBuf,
};

use antlr_rust::{
    InputStream,
    common_token_stream::CommonTokenStream,
};

use suzaku_extension_sdk::{
    language::{
        parser::{
            LanguageParserPolicy, 
            LanguageParsePolicyInfo,
            LanguageParserPolicyError,
            LanguageParserListener,
            LanguageParseResult,
        },
        meta::{
            IMeta,
            Metadata
        },
        meta_type::MetaType, 
        reorganzier::LanguageMetaReorganizePolicy
    },
    METADATA_FILE_EXTENSION,
};

use super::{
    generated::{
        javalexer::JavaLexer,
        javaparser::*,
    },
    meta::JavaMetaReorganizePolicy,
    parser_listener::ParserListener,
};

pub const SRC_FILE_EXTENSION: &str = "java";

#[derive(Debug, Clone, Copy)]
pub struct JavaParserPolicy;

impl JavaParserPolicy {
    fn parse(&self, src: &PathBuf) -> Option<Metadata> {
        if src.is_file() && src.exists() {
            let content = fs::read_to_string(src).expect("should read context of file");
            let data = InputStream::new(content.trim());
    
            let lexer = JavaLexer::new(data);
            let token_source = CommonTokenStream::new(lexer);
    
            let mut file_node = Metadata::new(MetaType::File);
            file_node.set_attr(src.to_str().unwrap());

            let reorganizer = JavaMetaReorganizePolicy::new();
            let parser_listener: ParserListener = ParserListener::new(file_node, Some(Box::new(reorganizer)));
    
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

    fn execute(&self, src: &PathBuf, output_dir: &PathBuf) -> LanguageParseResult<PathBuf> {
        if let Some(tree) = self.parse(src) {
            if !output_dir.exists() {
                _ = fs::create_dir_all(&output_dir);
            }

            let output_file_path = output_dir.join(format!("{}.{}", src.file_stem().unwrap().to_str().unwrap(), METADATA_FILE_EXTENSION));
            if let Ok(mut f) = File::create(&output_file_path) {
                let _ = f.write_all(tree.dump().unwrap().as_bytes());
                let _ = f.flush();
            }
            return Ok(output_file_path);
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
