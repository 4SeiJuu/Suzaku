use core::fmt::Debug;
use serde::Serialize;
use serde_json::Result;
use std::collections::LinkedList;

use suzaku_extension_sdk::language::inode::INode;

#[derive(Debug, PartialEq, Copy, Clone, Serialize)]
pub enum Java9NodeType {
    File,

    Operator,
    Keyword,
}

#[derive(Debug, Serialize, Clone)]
pub struct Java9Node {
    node_type: Java9NodeType,
    attr: Option<String>,
    members: LinkedList<Self>,
}

impl INode<Java9NodeType> for Java9Node {
    fn new(node_type: Java9NodeType) -> Self {
        Java9Node {
            node_type: node_type,
            attr: None,
            members: LinkedList::new(),
        }
    }

    fn get_node_type(&self) -> Java9NodeType {
        self.node_type
    }

    fn get_attr(&self) -> &Option<String> {
        &self.attr
    }

    fn set_attr(&mut self, value: &str) {
        self.attr = Some(String::from(value));
    }

    fn get_members(&self) -> &LinkedList<Java9Node> {
        &self.members
    }

    fn get_members_mut(&mut self) -> &mut LinkedList<Java9Node> {
        &mut self.members
    }

    fn dump(&self) -> Result<String> {
        serde_json::to_string(self)
    }
}

impl Java9Node {
    pub fn reorganize_children(&mut self) {
    }

    fn reorganize_special_node<'a>(&mut self, candidate_children: &'a mut LinkedList<Java9Node>) -> Option<&'a mut LinkedList<Java9Node>> {
        None
    }

    fn reorganize_children_of_import_declaration_node<'a>(children: &'a mut LinkedList<Java9Node>) -> Option<&'a mut LinkedList<Java9Node>> {
        None
    }

    fn reorganize_children_of_variable_initializer_node<'a>(chidlren: &'a mut LinkedList<Java9Node>) -> Option<&'a mut LinkedList<Java9Node>> {
        None
    }

    fn reorganize_children_of_type_declaration_node<'a>(children: &'a mut LinkedList<Java9Node>) -> Option<&'a mut LinkedList<Java9Node>> {
        None
    }
}
