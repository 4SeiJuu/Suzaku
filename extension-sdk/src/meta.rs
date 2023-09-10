use std::{
    collections::{
        LinkedList, 
        HashMap
    }, 
    str::FromStr, 
    fmt::Debug
};

use serde::{
    Serialize, 
    Deserialize
};
use serde_json::error::Result;

pub trait IMetaType : Sized {
    fn get_name(&self) -> String;
}

pub trait IMetadata<M>: Sized + Debug
where M: IMetaType + ToString + FromStr + Serialize + Eq + Clone {
    fn new(meta_type: M) -> Self;
    fn get_meta_type(&self) -> M;
    fn get_attrs(&self) -> &HashMap<String, String>;
    fn get_attrs_mut(&mut self) -> &mut HashMap<String, String>;
    fn get_attr(&self, key: &str) -> Option<&String>;
    fn set_attr(&mut self, key: &str, value: &str);
    fn get_members(&self) -> &LinkedList<Self>;
    fn get_members_mut(&mut self) -> &mut LinkedList<Self>;
    fn dump(&self) -> Result<String>;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metadata<M>
where M: IMetaType + ToString + FromStr + Serialize + Debug + Eq + Clone {
    meta_type: M,
    attrs: HashMap<String, String>,
    members: LinkedList<Self>,
}

impl<M> IMetadata<M> for Metadata<M>
where M: IMetaType + ToString + FromStr + Serialize + Debug + Eq + Clone {
    fn new(meta_type: M) -> Self {
        Metadata {
            meta_type,
            attrs: HashMap::new(),
            members: LinkedList::new(),
        }
    }

    fn get_meta_type(&self) -> M {
        self.meta_type.clone()
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