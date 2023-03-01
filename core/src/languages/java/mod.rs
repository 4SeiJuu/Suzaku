mod generated;
mod parser_listener;
mod java_node;

use std::{
    fs,
    fs::File,
    io::Write,
    path::Path,
};

use antlr_rust::{
    InputStream,
    common_token_stream::CommonTokenStream
};

use super::{
    Analyzer, 
    Result, 
    AnalyzerError,
    PARSED_RESULT_FOLDER_NAME,
};
use java_node::JavaNode;

use generated::javalexer::JavaLexer;
use generated::javaparser::*;
use parser_listener::ParserListener;
use super::inode::INode;

pub struct JavaAnalyzer {
}

impl JavaAnalyzer {
    fn parse(&self, src: &str) -> Option<JavaNode> {
        let content = fs::read_to_string(Path::new(&String::from(src))).expect("should read context of file");
        let data = InputStream::new(content.as_str());

        let lexer = JavaLexer::new(data);
        let token_source = CommonTokenStream::new(lexer);

        let mut parser_listener: ParserListener = ParserListener::new();
        let mut file_node = JavaNode::new(java_node::JavaNodeType::File);
        file_node.set_attr(src);
        parser_listener.stack_mut().push(file_node);

        let mut parser = JavaParser::new(token_source);
        let _listener_id = parser.add_parse_listener(Box::new(parser_listener));

        let result = match parser.compilationUnit() {
            Ok(_) => {
                Some(parser.remove_parse_listener(_listener_id).stack().top().unwrap().clone())
            },
            _ => None
        };
        result
    }

    fn analysis(&self, tree: &JavaNode) {

    }
}

impl<'consumer> Analyzer for JavaAnalyzer {
    fn new() -> Self {
        JavaAnalyzer {}
    }

    fn execute(&self, src: &str, output_dir: &str) -> Result<String> {
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

            println!("=========================================");
            println!("{}", "succeed to parse java file");
            println!("=========================================");
            println!("Result Json File: {}", output_file_path.to_str().unwrap());
        }
        Err(AnalyzerError {  })
    }
}
