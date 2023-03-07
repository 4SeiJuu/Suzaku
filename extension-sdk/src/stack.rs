use std::collections::LinkedList;
use serde::Serialize;
use serde_json::error::Result;

#[derive(Debug, Serialize)]
pub struct Stack<T: Serialize>(LinkedList<T>);

impl<T: Serialize> Stack<T> {
    pub fn new() -> Self {
        Stack(LinkedList::new())
    }

    pub fn push(&mut self, node: T) {
        self.0.push_back(node)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop_back()
    }

    pub fn top(&self) -> Option<&T> {
        self.0.back()
    }

    pub fn top_mut(&mut self) -> Option<&mut T> {
        self.0.back_mut()
    }

    pub fn dump(&self) -> Result<String> {
        serde_json::to_string(self)
    }
}