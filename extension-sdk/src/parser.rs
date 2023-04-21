use std::path::PathBuf;

use crate::stack::Stack;

use super::{
    reorganzier::LanguageMetaReorganizePolicy, 
    meta::{
        IMeta,
        Metadata,
    },
    meta_type::MetaType
};

#[derive(Debug)]
pub struct LanguageParserPolicyError {}
pub type LanguageParseResult<T> = std::result::Result<T, LanguageParserPolicyError>;

pub trait LanguageParserPolicy {
    fn new() -> Self;
    fn execute(&self, folder: &PathBuf, relative_file_path: &PathBuf) -> LanguageParseResult<String>;
}

pub trait LanguageParsePolicyInfo {
    fn get_filename_extensions(&self) -> Option<Vec<String>>;
}

pub trait LanguageParserListener {
    fn new(root_node: Metadata, reorganizer: Option<Box<dyn LanguageMetaReorganizePolicy>>) -> Self;
    fn results(&mut self) -> Option<Metadata>;
    fn stack(&self) -> &Stack<Metadata>;
    fn stack_mut(&mut self) -> &mut Stack<Metadata>;
    fn reorganizer(&mut self) -> Option<&mut dyn LanguageMetaReorganizePolicy>;

    fn update_node_attrs<T: Fn(&mut Metadata)>(&mut self, node_type: MetaType, update_attrs: T) -> Option<&Metadata> {
        match self.stack().top() {
            Some(top) => if top.get_node_type() != node_type {
                panic!("[ERROR] invalid node type. expected: {:?}, actual: {:?}\n== Dump ======\n{}\n===============", 
                    node_type, top.get_node_type(), self.stack().dump().unwrap());
            },
            None => panic!("[ERROR] invalid stack status. stack should not be empty.")
        };

        match self.stack_mut().top_mut() {
            Some(top_node) => {
                update_attrs(top_node);
                Some(top_node)
            },
            None => panic!("[ERROR] invalid status. parent node not found.")
        }
    }

    fn add_to_parent_member(&mut self) {
        let mut poped_node = self
            .stack_mut()
            .pop()
            .unwrap_or_else(|| panic!("[ERROR] invalid status. top node not found."));
        
        let children = match &mut self.reorganizer() {
            Some(reorg) => reorg.reorganize(&mut poped_node),
            None => vec![poped_node]
        };

        let parent = self
            .stack_mut()
            .top_mut()
            .unwrap_or_else(|| panic!("[ERROR] invalid status. parent node not found."));
        for child in children {
            parent.get_members_mut().push_back(child);
        }
    }
}