use core::fmt::Debug;

use serde::{Serialize, Deserialize};
use serde_json::Result;
use std::{
    collections::LinkedList
};

use suzaku_extension_sdk::language::inode::INode;
use super::node_type::JavaNodeType;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JavaNode {
    node_type: JavaNodeType,
    attr: Option<String>,
    members: LinkedList<Self>,
}

impl INode<JavaNodeType> for JavaNode {
    fn new(node_type: JavaNodeType) -> Self {
        JavaNode {
            node_type: node_type,
            attr: None,
            members: LinkedList::new(),
        }
    }

    fn get_node_type(&self) -> JavaNodeType {
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

impl JavaNode {
    pub fn reorganize(&mut self) -> Vec<JavaNode> {
        let mut children: Vec<JavaNode> = Vec::new();

        if self.get_members().len() > 0 {
            match self.get_node_type() {
                JavaNodeType::Expression | JavaNodeType::Primary | JavaNodeType::Literal | JavaNodeType::VariableDeclarators => {
                    if self.get_members().len() == 1 {
                        children.push(self.get_members_mut().pop_front().unwrap());
                        return children;
                    }
    
                    if self.get_members().front().unwrap().get_node_type() == JavaNodeType::Operator {
                        children.push(self.get_members_mut().pop_front().unwrap());
                    }
                },
                JavaNodeType::Arguments | JavaNodeType::FormalParameters => {
                    if self.get_members().len() == 1 {
                        let mut unique_member = self.get_members_mut().pop_front().unwrap();
                        self.get_members_mut().append(unique_member.get_members_mut());
                    }
                },
                JavaNodeType::TypeDeclaration | JavaNodeType::ClassBodyDeclaration | JavaNodeType::MemberDeclaration => {
                    let mut top = self.get_members_mut().pop_back().unwrap();
                    while let Some(front) = self.get_members_mut().pop_back() {
                        top.get_members_mut().push_front(front);
                    }
                    children.push(top);
                    return children;
                },
                JavaNodeType::InterfaceCommonBodyDeclaration => {
                    while let Some(member) = self.get_members_mut().pop_front() {
                        children.push(member);
                    }
                },
                _ => ()
            }
    
            self.reorganize_for_method_call();
            match self.get_node_type() {
                JavaNodeType::Expression | JavaNodeType::Statement => {
                    if self.get_members().len() == 1 && self.get_members_mut().front().unwrap().get_node_type() == JavaNodeType::MethodCall  {
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
        let mut temp_children: LinkedList<JavaNode> = LinkedList::new();

        for member in self.get_members().iter().rev() {
            if operate_method_call {
                if member.get_node_type() == JavaNodeType::Operator || member.get_node_type() == JavaNodeType::Separator {
                    operate_method_call = false;
                    temp_children.push_front(member.clone());
                    continue;
                }
                temp_children.front_mut().unwrap().get_members_mut().push_front(member.clone());
            } else {
                temp_children.push_front(member.clone());
                if member.get_node_type() == JavaNodeType::MethodCall {
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
