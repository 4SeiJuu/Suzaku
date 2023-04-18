use core::fmt::Debug;

use serde::{Serialize, Deserialize};
use serde_json::Result;
use std::{
    collections::LinkedList
};

use suzaku_extension_sdk::language::{
    meta::IMeta,
    meta_type::MetaType
};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JavaMeta {
    node_type: MetaType,
    attr: Option<String>,
    members: LinkedList<Self>,
}

impl IMeta<MetaType> for JavaMeta {
    fn new(node_type: MetaType) -> Self {
        JavaMeta {
            node_type: node_type,
            attr: None,
            members: LinkedList::new(),
        }
    }

    fn get_node_type(&self) -> MetaType {
        self.node_type
    }

    fn get_attr(&self) -> &Option<String> {
        &self.attr
    }

    fn set_attr(&mut self, value: &str) {
        self.attr = Some(String::from(value));
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

impl JavaMeta {
    pub fn reorganize(&mut self) -> Vec<JavaMeta> {
        let mut children: Vec<JavaMeta> = Vec::new();

        if self.get_members().len() > 0 {
            match self.get_node_type() {
                MetaType::Expression | MetaType::Primary | MetaType::Literal | MetaType::VariableDeclarators => {
                    if self.get_members().len() == 1 {
                        children.push(self.get_members_mut().pop_front().unwrap());
                        return children;
                    }
    
                    if self.get_members().front().unwrap().get_node_type() == MetaType::Operator {
                        children.push(self.get_members_mut().pop_front().unwrap());
                    }
                },
                MetaType::Arguments | MetaType::FormalParameters => {
                    if self.get_members().len() == 1 {
                        let mut unique_member = self.get_members_mut().pop_front().unwrap();
                        self.get_members_mut().append(unique_member.get_members_mut());
                    }
                },
                MetaType::TypeDeclaration | MetaType::ClassBodyDeclaration | MetaType::MemberDeclaration => {
                    let mut top = self.get_members_mut().pop_back().unwrap();
                    while let Some(front) = self.get_members_mut().pop_back() {
                        top.get_members_mut().push_front(front);
                    }
                    children.push(top);
                    return children;
                },
                MetaType::InterfaceCommonBodyDeclaration => {
                    while let Some(member) = self.get_members_mut().pop_front() {
                        children.push(member);
                    }
                },
                _ => ()
            }
    
            self.reorganize_for_method_call();
            match self.get_node_type() {
                MetaType::Expression | MetaType::Statement => {
                    if self.get_members().len() == 1 && self.get_members_mut().front().unwrap().get_node_type() == MetaType::MethodCall  {
                        children.push(self.get_members_mut().pop_front().unwrap());
                        return children;
                    }
                },
                _ => ()
            }
        }

        children.push(self.clone());
        children
    }

    pub fn reorganize_for_method_call(&mut self) {
        let mut operate_method_call = false;
        let mut temp_children: LinkedList<JavaMeta> = LinkedList::new();

        for member in self.get_members().iter().rev() {
            if operate_method_call {
                if member.get_node_type() == MetaType::Operator || member.get_node_type() == MetaType::Separator {
                    operate_method_call = false;
                    temp_children.push_front(member.clone());
                    continue;
                }
                temp_children.front_mut().unwrap().get_members_mut().push_front(member.clone());
            } else {
                temp_children.push_front(member.clone());
                if member.get_node_type() == MetaType::MethodCall {
                    operate_method_call = true;
                }
            }
        }
        self.get_members_mut().clear();
        self.get_members_mut().append(&mut temp_children);
    }

    pub fn reorganize_for_expression(&mut self) {

    }
}
