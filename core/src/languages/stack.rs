use std::collections::LinkedList;
use serde::Serialize;
use super::inode::Node;

#[derive(Debug, Serialize)]
pub struct Stack(LinkedList<Node>);

impl Stack {
    pub fn new() -> Self {
        Stack(LinkedList::new())
    }

    pub fn push(&mut self, node: Node) {
        self.0.push_back(node)
    }

    pub fn pop(&mut self) -> Option<Node> {
        self.0.pop_back()
    }

    pub fn top(&self) -> Option<&Node> {
        self.0.back()
    }

    pub fn top_mut(&mut self) -> Option<&mut Node> {
        self.0.back_mut()
    }
}