use core::fmt::Debug;

use serde::Serialize;
use serde_json::Result;
use std::{
    cell::RefCell, 
    collections::LinkedList
};

use suzaku_extension_sdk::language::inode::INode;
use super::java_node_type::JavaNodeType;


#[derive(Debug, Serialize, Clone)]
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
    pub fn reorganize_children(&mut self) {
        let self_rc = RefCell::new(self);
        let mut self_rf = self_rc.borrow_mut();

        let children = self_rf.members.clone();
        if children.is_empty() {
            return;
        }

        let mut valid_children = LinkedList::<JavaNode>::new();
        for mut child in children {
            child.reorganize_children();

            match child.get_node_type() {
                JavaNodeType::RecordDeclaration
                | JavaNodeType::MethodDeclaration
                | JavaNodeType::GenericMethodDeclaration
                | JavaNodeType::FieldDeclaration
                | JavaNodeType::ConstructorDeclaration
                | JavaNodeType::GenericConstructorDeclaration
                | JavaNodeType::InterfaceDeclaration
                | JavaNodeType::AnnotationTypeDeclaration
                | JavaNodeType::ClassDeclaration
                | JavaNodeType::EnumDeclaration => {
                    valid_children.push_back(child.clone());
                    continue;
                }
                _ => (),
            };

            let ctx_opt = self_rf.clone().attr;
            let child_ctx_opt = child.attr.clone();
            if ctx_opt == child_ctx_opt {
                if !child.get_members().is_empty() {
                    valid_children.append(child.get_members_mut());
                }
            } else {
                valid_children.push_back(child.clone());
            }
        }

        self_rf.members.clear();

        if let Some(mut reorganized_children) = self_rf.reorganize_special_node(&mut valid_children)
        {
            self_rf.members.append(&mut reorganized_children);
        }
    }

    fn reorganize_special_node<'a>(&mut self, candidate_children: &'a mut LinkedList<JavaNode>) -> Option<&'a mut LinkedList<JavaNode>> {
        match self.get_node_type() {
            JavaNodeType::ImportDeclaration | JavaNodeType::PackageDeclaration => Self::reorganize_children_of_import_declaration_node(candidate_children),
            JavaNodeType::TypeDeclaration => Self::reorganize_children_of_type_declaration_node(candidate_children),
            JavaNodeType::VariableInitializer | JavaNodeType::ExpressionList => Self::reorganize_children_of_variable_initializer_node(candidate_children),
            _ => Some(candidate_children),
        }
    }

    fn reorganize_children_of_import_declaration_node<'a>(children: &'a mut LinkedList<JavaNode>) -> Option<&'a mut LinkedList<JavaNode>> {
        let cloned = children.clone();
        children.clear();
        for mut node in cloned {
            if node.get_node_type() == JavaNodeType::QualifiedName {
                let mut items: Vec<String> = Vec::new();
                while let Some(child) = node.get_members_mut().pop_front() {
                    if let Some(attr) = child.get_attr().clone() {
                        items.push(attr);
                    }
                }
                node.set_attr(items.join(".").as_str())
            }
            children.push_back(node);
        }
        Some(children)
    }

    fn reorganize_children_of_variable_initializer_node<'a>(chidlren: &'a mut LinkedList<JavaNode>) -> Option<&'a mut LinkedList<JavaNode>> {
        if chidlren.len() == 2
            && chidlren.front().unwrap().get_node_type() == JavaNodeType::Expression
            && chidlren.back().unwrap().get_node_type() == JavaNodeType::MethodCall
        {
            let expression_node = chidlren.pop_front().unwrap();
            let mut method_call_node = chidlren.pop_back().unwrap();
            for member in method_call_node.get_members_mut() {
                if member.get_node_type() == JavaNodeType::Identifier {
                    member.set_attr(
                        format!(
                            "{}.{}",
                            expression_node.get_attr().as_ref().unwrap(),
                            member.get_attr().as_ref().unwrap()
                        )
                        .as_str(),
                    );
                }
            }
            chidlren.push_back(method_call_node);
            return Some(chidlren);
        }
        Some(chidlren)
    }

    fn reorganize_children_of_type_declaration_node<'a>(children: &'a mut LinkedList<JavaNode>) -> Option<&'a mut LinkedList<JavaNode>> {
        if let Some(mut class_declaration) = children.pop_back() {
            while let Some(child) = children.pop_back() {
                assert_eq!(child.get_node_type(), JavaNodeType::ClassOrInterfaceModifier);
                if let Some(attr) = child.get_attr() {
                    let mut modifier = JavaNode::new(JavaNodeType::Modifier);
                    modifier.set_attr(attr);
                    class_declaration.get_members_mut().push_front(modifier);
                }
            }
            children.push_back(class_declaration);
        }

        Some(children)
    }
}
