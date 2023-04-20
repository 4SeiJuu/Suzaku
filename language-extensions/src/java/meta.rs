use std::{
    collections::LinkedList
};

use suzaku_extension_sdk::{
    meta::{
        IMeta,
        Metadata
    },
    meta_type::MetaType, 
    reorganzier::LanguageMetaReorganizePolicy
};

pub struct JavaMetaReorganizePolicy;

impl JavaMetaReorganizePolicy {
    pub fn reorganize_for_method_call(&mut self, meta: &mut Metadata) {
        let mut operate_method_call = false;
        let mut temp_children: LinkedList<Metadata> = LinkedList::new();

        for member in meta.get_members().iter().rev() {
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
        meta.get_members_mut().clear();
        meta.get_members_mut().append(&mut temp_children);
    }

    pub fn reorganize_for_expression(&mut self) {

    }
}

impl LanguageMetaReorganizePolicy for JavaMetaReorganizePolicy {
    fn new() -> Self {
        JavaMetaReorganizePolicy {}
    }

    fn reorganize(&mut self, meta: &mut Metadata) -> Vec<Metadata> {
        let mut children: Vec<Metadata> = Vec::new();

        if meta.get_members().len() > 0 {
            match meta.get_node_type() {
                MetaType::Expression | MetaType::Primary | MetaType::Literal | MetaType::VariableDeclarators => {
                    if meta.get_members().len() == 1 {
                        children.push(meta.get_members_mut().pop_front().unwrap());
                        return children;
                    }
    
                    if meta.get_members().front().unwrap().get_node_type() == MetaType::Operator {
                        children.push(meta.get_members_mut().pop_front().unwrap());
                    }
                },
                MetaType::Arguments | MetaType::FormalParameters => {
                    if meta.get_members().len() == 1 {
                        let mut unique_member = meta.get_members_mut().pop_front().unwrap();
                        meta.get_members_mut().append(unique_member.get_members_mut());
                    }
                },
                MetaType::TypeDeclaration | MetaType::ClassBodyDeclaration | MetaType::MemberDeclaration => {
                    let mut top = meta.get_members_mut().pop_back().unwrap();
                    while let Some(front) = meta.get_members_mut().pop_back() {
                        top.get_members_mut().push_front(front);
                    }
                    children.push(top);
                    return children;
                },
                MetaType::InterfaceCommonBodyDeclaration => {
                    while let Some(member) = meta.get_members_mut().pop_front() {
                        children.push(member);
                    }
                },
                _ => ()
            }
    
            self.reorganize_for_method_call(meta);
            match meta.get_node_type() {
                MetaType::Expression | MetaType::Statement => {
                    if meta.get_members().len() == 1 && meta.get_members_mut().front().unwrap().get_node_type() == MetaType::MethodCall  {
                        children.push(meta.get_members_mut().pop_front().unwrap());
                        return children;
                    }
                },
                _ => ()
            }
        }

        children.push(meta.clone());
        children
    }

}
