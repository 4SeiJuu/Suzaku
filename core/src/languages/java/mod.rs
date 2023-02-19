mod generated;
mod listener;

use std::{
    fs,
    path::Path
};

use antlr_rust::{
    InputStream,
    common_token_stream::CommonTokenStream
};

use super::Analyzer;
use super::inode::Node;

use generated::javalexer::JavaLexer;
use generated::javaparser::*;
use listener::Listener;

pub struct JavaAnalyzer {
}

impl JavaAnalyzer {
}

impl<'consumer> Analyzer for JavaAnalyzer {
    fn new() -> Self {
        JavaAnalyzer {}
    }

    fn run(&self, src: &str) {
        let content = fs::read_to_string(Path::new(&String::from(src))).expect("should read context of file");
        let data = InputStream::new(content.as_str());

        let lexer = JavaLexer::new(data);
        let token_source = CommonTokenStream::new(lexer);

        let mut parser = JavaParser::new(token_source);

        let mut listener: Listener = Listener::new();
        let mut file_node = Node::new(super::inode::NodeType::File);
        file_node.set_attr("path", src);
        listener.get_stack().push_back(file_node);

        let listener_id = parser.add_parse_listener(Box::new(listener));

        match parser.compilationUnit() {
            Ok(_) => {
                println!("{}", "succeed to parse java file");
                println!("=========================================");
                println!("{:?}", serde_json::to_string(parser.remove_parse_listener(listener_id).get_stack()));
            },
            Err(error) => {
                println!("{}", error);
            }
        };
    }
}


