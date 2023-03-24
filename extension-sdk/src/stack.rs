use serde::Serialize;
use serde_json::error::Result;

#[derive(Debug, Serialize)]
pub struct Stack<T: Serialize>(Vec<T>);

impl<T: Serialize> Stack<T> {
    pub fn new() -> Self {
        Stack(Vec::new())
    }

    pub fn empty(&self) -> bool {
        self.0.len() == 0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, node: T) {
        self.0.push(node)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    pub fn top(&self) -> Option<&T> {
        self.0.last()
    }

    pub fn top_mut(&mut self) -> Option<&mut T> {
        self.0.last_mut()
    }

    pub fn get_by_index(&self, index: usize) -> Option<&T> {
        if index < 0 {
            return None
        }
        self.0.get(index)
    }

    pub fn dump(&self) -> Result<String> {
        serde_json::to_string(self)
    }
}