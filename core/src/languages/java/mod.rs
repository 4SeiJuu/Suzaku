mod generated;
mod parser_listener;

use std::{
    fs,
    path::Path,
};

use antlr_rust::{
    InputStream,
    common_token_stream::CommonTokenStream
};

use super::{Analyzer, Result, AnalyzerError};
use super::inode::ContextNode;

use generated::javalexer::JavaLexer;
use generated::javaparser::*;
use parser_listener::ParserListener;

pub struct JavaAnalyzer {
}

impl JavaAnalyzer {
}

impl<'consumer> Analyzer for JavaAnalyzer {
    fn new() -> Self {
        JavaAnalyzer {}
    }

    fn analysis(&self, src: &str) -> Result<String> {
        let content = fs::read_to_string(Path::new(&String::from(src))).expect("should read context of file");
        let data = InputStream::new(content.as_str());

        let lexer = JavaLexer::new(data);
        let token_source = CommonTokenStream::new(lexer);

        let mut parser_listener: ParserListener = ParserListener::new();
        let mut file_node = ContextNode::new(super::inode::NodeType::File);
        file_node.set_attr(src);
        parser_listener.stack_mut().push(file_node);

        let mut parser = JavaParser::new(token_source);
        let _listener_id = parser.add_parse_listener(Box::new(parser_listener));

        let analysis_results = parser.compilationUnit();
        match analysis_results {
            Ok(_) => {
                println!("=========================================");
                println!("{}", "succeed to parse java file");
                println!("=========================================");
                
                Ok(parser.remove_parse_listener(_listener_id).stack().dump().unwrap())
            },
            Err(error) => {
                println!("{}", error);
                Err(AnalyzerError {  })
            }
        }
    }
}


