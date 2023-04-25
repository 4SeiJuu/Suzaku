use std::{
    collections::{
        LinkedList, 
        HashMap
    }, 
    str::FromStr
};

use serde::{
    Serialize, 
    Deserialize
};
use serde_json::error::Result;

use super::meta_type::MetaType;

pub trait IMeta<T>: Sized 
where T: ToString + FromStr {
    fn new(meta_type: T) -> Self;
    fn get_node_type(&self) -> T;
    fn get_attrs(&self) -> &HashMap<String, String>;
    fn get_attrs_mut(&mut self) -> &mut HashMap<String, String>;
    fn get_attr(&self, key: &str) -> Option<&String>;
    fn set_attr(&mut self, key: &str, value: &str);
    fn get_members(&self) -> &LinkedList<Self>;
    fn get_members_mut(&mut self) -> &mut LinkedList<Self>;
    fn dump(&self) -> Result<String>;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metadata {
    node_type: MetaType,
    attrs: HashMap<String, String>,
    members: LinkedList<Self>,
}

impl IMeta<MetaType> for Metadata {
    fn new(node_type: MetaType) -> Self {
        Metadata {
            node_type: node_type,
            attrs: HashMap::new(),
            members: LinkedList::new(),
        }
    }

    fn get_node_type(&self) -> MetaType {
        self.node_type
    }

    fn get_attrs(&self) -> &HashMap<String, String> {
        &self.attrs
    }

    fn get_attrs_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.attrs
    }

    fn get_attr(&self, key: &str) -> Option<&String> {
        self.attrs.get(&String::from(key)).clone()
    }

    fn set_attr(&mut self, key: &str, value: &str) {
        self.attrs.insert(String::from(key), String::from(value));
    }

    fn get_members(&self) -> &LinkedList<Self> {
        &self.members
    }

    fn get_members_mut(&mut self) -> &mut LinkedList<Self> {
        &mut self.members
    }

    fn dump(&self) -> Result<String> {
        serde_json::to_string(self)
    }
}