pub mod parser;
pub mod parser_listener;
pub mod reorganzier;
pub mod extractor;
pub mod mapper;
pub mod analyzer;
pub mod reporter;

pub mod meta;
pub mod element;
pub mod stack;
pub mod utils;

// for parser
pub const METADATA_FOLDER_NAME: &str = "metadata";
pub const METADATA_FILE_EXTENSION: &str = "json";

// for data clean
pub const ELEMENT_FOLDER_NAME: &str = "elements";
pub const ELEMENT_FILE_EXTENSION: &str = "ele";

// for analysis
pub const GRAPH_FOLDER_NAME: &str = "graph";
pub const GRAPH_FILE_EXTENSION: &str = "dot";

// for reporter
pub const GRAPHVIZ_DOT_FILE_EXTENSION: &str = "dot";

pub const ATTR_EXPRESSION: &str = "EXPRESSION";
pub const ATTR_FOLDER: &str = "FOLDER";
pub const ATTR_FILE: &str = "FILE";
pub const ATTR_HASH_CODE: &str = "HASH";

pub const SRC_FILE_EXTENSION: &str = "java";

