use std::collections::{LinkedList, HashMap};
use core::fmt::Debug;
use serde::Serialize;

#[derive(Debug, PartialEq, Copy, Clone, Serialize)]
pub enum NodeType {
    File,
    Package,
    Import,
    Type,
    Extends,
    Implements,
    Permits,
    ClassBody,
}

#[derive(Debug, Serialize)]
pub struct ContextNode {
    node_type: NodeType,
    attrs: HashMap<String, String>,
    members: LinkedList<Self>
}

impl ContextNode {
    pub fn new(node_type: NodeType) -> Self {
        ContextNode { node_type: node_type, attrs: HashMap::new(), members: LinkedList::new() }
    }

    pub fn get_node_type(&self) -> NodeType {
        self.node_type
    }

    pub fn get_attrs(&self) -> &HashMap<String, String> {
        &self.attrs
    }

    pub fn set_attr(&mut self, key: &str, value: &str) {
        self.attrs.insert(String::from(key), String::from(value));
    }

    pub fn get_members_mut(&mut self) -> &mut LinkedList<ContextNode> {
        &mut self.members
    }
}
