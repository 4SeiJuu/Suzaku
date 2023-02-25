use std::collections::{LinkedList, HashMap};
use core::fmt::Debug;
use serde::Serialize;

#[derive(Debug, PartialEq, Copy, Clone, Serialize)]
pub enum NodeType {
    File,
    Package,            // packageDeclaration
    Import,             // importDeclaration
    Type,               // typeDeclaration
    Extends,
    Implements,
    Permits,
    ClassBody,          // classBody
    ClassMember,        // classBodyDeclaration
    ClassMemberDef,     // memberDeclaration
}

#[derive(Debug, Serialize)]
pub struct ContextNode {
    node_type: NodeType,
    attrs: HashMap<String, Vec<String>>,
    members: LinkedList<Self>
}

impl ContextNode {
    pub fn new(node_type: NodeType) -> Self {
        ContextNode { node_type: node_type, attrs: HashMap::new(), members: LinkedList::new() }
    }

    pub fn get_node_type(&self) -> NodeType {
        self.node_type
    }

    pub fn get_attrs(&self) -> &HashMap<String, Vec<String>> {
        &self.attrs
    }

    pub fn set_attr(&mut self, key: &str, value: Vec<String>) {
        self.attrs.insert(String::from(key), value);
    }

    pub fn add_attr_value(&mut self, key: &str, value: &str) {
        if self.attrs.contains_key(key) {
            self.attrs.get_mut(key).unwrap().push(String::from(value));
        } else {
            self.attrs.insert(String::from(key), vec![String::from(value)]);
        }
    }

    pub fn get_members_mut(&mut self) -> &mut LinkedList<ContextNode> {
        &mut self.members
    }
}
