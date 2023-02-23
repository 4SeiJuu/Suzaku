use std::collections::LinkedList;
use serde::Serialize;
use serde_json::error::Result;
use super::inode::ContextNode;

#[derive(Debug, Serialize)]
pub struct Stack(LinkedList<ContextNode>);

impl Stack {
    pub fn new() -> Self {
        Stack(LinkedList::new())
    }

    pub fn push(&mut self, node: ContextNode) {
        self.0.push_back(node)
    }

    pub fn pop(&mut self) -> Option<ContextNode> {
        self.0.pop_back()
    }

    pub fn top(&self) -> Option<&ContextNode> {
        self.0.back()
    }

    pub fn top_mut(&mut self) -> Option<&mut ContextNode> {
        self.0.back_mut()
    }

    pub fn dump(&self) -> Result<String> {
        serde_json::to_string(self)
    }
}