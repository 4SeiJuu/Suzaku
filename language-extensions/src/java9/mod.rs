mod generated;
mod parser_listener;
mod java_node;

use std::path::PathBuf;
use std
::{
    fs,
    fs::File,
    io::Write,
    path::Path,
};
use regex::Regex;

use antlr_rust::{
    InputStream,
    common_token_stream::CommonTokenStream,
};

use suzaku_extension_sdk::language::{
    parser::{
        LanguageParserPolicy, 
        Result, 
        LanguageParserPolicyError,
    },
    inode::INode,
};

use generated::java9lexer::Java9Lexer;
use generated::java9parser::*;
use java_node::Java9Node;
use parser_listener::ParserListener;
use crate::PARSED_RESULT_FOLDER_NAME;

pub const SRC_FILE_EXTENSION: &str = "java";

pub struct Java9ParserPolicy {
}

impl Java9ParserPolicy {
    fn parse(&self, src: &PathBuf) -> Option<Java9Node> {
        let content = fs::read_to_string(src).expect("should read context of file");

        let re = Regex::new(r"/\*(.|\n)*?\*/").unwrap();
        let no_comments_content = re.replace_all(content.as_str(), "");

        let data = InputStream::new(no_comments_content.trim());

        let lexer = Java9Lexer::new(data);
        let token_source = CommonTokenStream::new(lexer);

        let mut parser_listener: ParserListener = ParserListener::new();
        let mut file_node = Java9Node::new(java_node::Java9NodeType::File);
        file_node.set_attr(src.to_str().unwrap());
        parser_listener.stack_mut().push(file_node);

        let mut parser = Java9Parser::new(token_source);
        let _listener_id = parser.add_parse_listener(Box::new(parser_listener));

        let result = match parser.compilationUnit() {
            Ok(_) => {
                Some(parser.remove_parse_listener(_listener_id).stack().top().unwrap().clone())
            },
            _ => None
        };
        result
    }

    fn analysis(&self, tree: &Java9Node) {

    }
}

impl<'consumer> LanguageParserPolicy for Java9ParserPolicy {
    fn new() -> Self {
        Java9ParserPolicy {}
    }

    fn execute(&self, src: &PathBuf, output_dir: &PathBuf) -> Result<String> {
        if let Some(tree) = self.parse(src) {
            let output_folder_path = Path::new(output_dir).join(PARSED_RESULT_FOLDER_NAME);
            if !output_folder_path.exists() {
                _ = fs::create_dir_all(&output_folder_path);
            }

            let src_file_path = Path::new(src);
            let output_file_path = output_folder_path.join(format!("{}.{}", src_file_path.file_stem().unwrap().to_str().unwrap(), "json"));
            if let Ok(mut f) = File::create(&output_file_path) {
                let _ = f.write_all(tree.dump().unwrap().as_bytes());
                let _ = f.flush();
            }
            return Ok(String::from(output_file_path.to_str().unwrap()));
        }
        Err(LanguageParserPolicyError {  })
    }

    fn get_filename_extension(&self) -> &str {
        SRC_FILE_EXTENSION
    }
}
