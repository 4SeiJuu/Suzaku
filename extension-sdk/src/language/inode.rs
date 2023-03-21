use std::{
    collections::LinkedList, str::FromStr
};
use serde_json::error::Result;

pub trait INode<T>: Sized 
where T: ToString + FromStr {
    fn new(node_type: T) -> Self;
    fn get_node_type(&self) -> T;
    fn get_attr(&self) -> &Option<String>;
    fn set_attr(&mut self, value: &str);
    fn get_members(&self) -> &LinkedList<Self>;
    fn get_members_mut(&mut self) -> &mut LinkedList<Self>;
    fn dump(&self) -> Result<String>;
}
