use std::{
    collections::LinkedList, 
    cell::RefCell
};
use core::fmt::Debug;
use serde::Serialize;

#[derive(Debug, PartialEq, Copy, Clone, Serialize)]
pub enum NodeType {
    File,
    PackageDeclaration,
    ImportDeclaration,
    TypeDeclaration,
    Modifier,
    ClassOrInterfaceModifier,
    VariableModifier,
    ClassDeclaration,
    TypeParameters,
    TypeParameter,
    TypeBound,
    EnumDeclaration,
    EnumConstants,
    EnumConstant,
    EnumBodyDeclarations,
    InterfaceDeclaration,
    ClassBody,
    InterfaceBody,
    ClassBodyDeclaration,
    MemberDeclaration,
    MethodDeclaration,
    MethodBody,
    TypeTypeOrVoid,
    GenericMethodDeclaration,
    GenericConstructorDeclaration,
    ConstructorDeclaration,
    CompactConstructorDeclaration,
    FieldDeclaration,
    InterfaceBodyDeclaration,
    InterfaceMemberDeclaration,
    ConstDeclaration,
    ConstantDeclarator,
    InterfaceMethodDeclaration,
    InterfaceMethodModifier,
    GenericInterfaceMethodDeclaration,
    InterfaceCommonBodyDeclaration,
    VariableDeclarators,
    VariableDeclarator,
    VariableDeclaratorId,
    VariableInitializer,
    ArrayInitializer,
    ClassOrInterfaceType,
    TypeArgument,
    QualifiedNameList,
    FormalParameters,
    ReceiverParameter,
    FormalParameterList,
    FormalParameter,
    LastFormalParameter,
    LambdaLVTIList,
    LambdaLVTIParameter,
    QualifiedName,
    Literal,
    IntegerLiteral,
    FloatLiteral,
    AltAnnotationQualifiedName,
    Annotation,
    ElementValuePairs,
    ElementValuePair,
    ElementValue,
    ElementValueArrayInitializer,
    AnnotationTypeDeclaration,
    AnnotationTypeBody,
    AnnotationTypeElementDeclaration,
    AnnotationTypeElementRest,
    AnnotationMethodOrConstantRest,
    AnnotationMethodRest,
    AnnotationConstantRest,
    DefaultValue,
    ModuleDeclaration,
    ModuleBody,
    ModuleDirective,
    RequiresModifier,
    RecordDeclaration,
    RecordHeader,
    RecordComponentList,
    RecordComponent,
    RecordBody,
    Block,
    BlockStatement,
    LocalVariableDeclaration,
    Identifier,
    TypeIdentifier,
    LocalTypeDeclaration,
    Statement,
    CatchClause,
    CatchType,
    FinallyBlock,
    ResourceSpecification,
    Resources,
    Resource,
    SwitchBlockStatementGroup,
    SwitchLabel,
    ForControl,
    ForInit,
    EnhancedForControl,
    ParExpression,
    ExpressionList,
    MethodCall,
    Expression,
    Pattern,
    LambdaExpression,
    LambdaParameters,
    LambdaBody,
    Primary,
    SwitchExpression,
    SwitchLabeledRule,
    GuardedPattern,
    SwitchRuleOutcome,
    ClassType,
    Creator,
    CreatedName,
    InnerCreator,
    ArrayCreatorRest,
    ClassCreatorRest,
    ExplicitGenericInvocation,
    TypeArgumentsOrDiamond,
    NonWildcardTypeArgumentsOrDiamond,
    NonWildcardTypeArguments,
    TypeList,
    TypeType,
    PrimitiveType,
    TypeArguments,
    SuperSuffix,
    ExplicitGenericInvocationSuffix,
    Arguments,

    Operator,
    Keyword,
}

#[derive(Debug, Serialize, Clone)]
pub struct ContextNode {
    node_type: NodeType,
    attr: Option<String>,
    members: LinkedList<Self>
}

impl ContextNode {
    pub fn new(node_type: NodeType) -> Self {
        ContextNode { node_type: node_type, attr: None, members: LinkedList::new() }
    }

    pub fn get_node_type(&self) -> NodeType {
        self.node_type
    }

    pub fn get_attr(&self) -> &Option<String> {
        &self.attr
    }

    pub fn set_attr(&mut self, value: &str) {
        self.attr = Some(String::from(value));
    }

    pub fn get_members(&self) -> &LinkedList<ContextNode> {
        &self.members
    }

    pub fn get_members_mut(&mut self) -> &mut LinkedList<ContextNode> {
        &mut self.members
    }

    pub fn reorganize_children(&mut self) {
        let self_rc = RefCell::new(self);
        let mut self_rf = self_rc.borrow_mut();

        let children = self_rf.members.clone();
        if children.is_empty() {
            return;
        }

        let mut valid_children = LinkedList::<ContextNode>::new();
        for mut child in children {
            child.reorganize_children();

            match child.get_node_type() {
                NodeType::RecordDeclaration | NodeType::MethodDeclaration | NodeType::GenericMethodDeclaration | NodeType::FieldDeclaration 
                | NodeType::ConstructorDeclaration | NodeType::GenericConstructorDeclaration | NodeType::InterfaceDeclaration 
                | NodeType::AnnotationTypeDeclaration | NodeType::ClassDeclaration | NodeType::EnumDeclaration => {
                    valid_children.push_back(child.clone());
                    continue
                },
                _ => ()
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

        if let Some(mut reorganized_children) = self_rf.reorganize_special_node(&mut valid_children) {
            self_rf.members.append(&mut reorganized_children);
        }
    }

    fn reorganize_special_node<'a>(&mut self, candidate_children: &'a mut LinkedList<ContextNode>) -> Option<&'a mut LinkedList<ContextNode>> {
        match self.get_node_type() {
            NodeType::ImportDeclaration | NodeType::PackageDeclaration => Self::reorganize_children_of_import_declaration_node(candidate_children),
            NodeType::TypeDeclaration => Self::reorganize_children_of_type_declaration_node(candidate_children),
            NodeType::VariableInitializer | NodeType::ExpressionList => Self::reorganize_children_of_variable_initializer_node(candidate_children),
            _ => Some(candidate_children)
        }
    }

    fn reorganize_children_of_import_declaration_node<'a>(children: &'a mut LinkedList<ContextNode>) -> Option<&'a mut LinkedList<ContextNode>> {
        let cloned = children.clone();
        children.clear();
        for mut node in cloned {
            if node.get_node_type() == NodeType::QualifiedName {
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

    fn reorganize_children_of_variable_initializer_node<'a>(chidlren: &'a mut LinkedList<ContextNode>) -> Option<&'a mut LinkedList<ContextNode>> {
        if chidlren.len() == 2 
            && chidlren.front().unwrap().get_node_type() == NodeType::Expression 
            && chidlren.back().unwrap().get_node_type() == NodeType::MethodCall {
                let expression_node = chidlren.pop_front().unwrap();
                let mut method_call_node = chidlren.pop_back().unwrap();
                for member in method_call_node.get_members_mut() {
                    if member.get_node_type() == NodeType::Identifier {
                        member.set_attr(format!("{}.{}", expression_node.get_attr().as_ref().unwrap(), member.get_attr().as_ref().unwrap()).as_str());
                    }
                }
                chidlren.push_back(method_call_node);
                return Some(chidlren);
            }
            Some(chidlren)
    }

    fn reorganize_children_of_type_declaration_node<'a>(children: &'a mut LinkedList<ContextNode>) -> Option<&'a mut LinkedList<ContextNode>> {
        assert_eq!(children.len(), 2);

        let class_or_interface_modifier = children.pop_front().unwrap();
        let mut modifier = ContextNode::new(NodeType::Modifier);
        if let Some(attr) = class_or_interface_modifier.get_attr() {
            modifier.set_attr(attr.as_str());
        }

        let mut declaration = children.pop_back().unwrap();
        declaration.get_members_mut().push_front(modifier);
        children.push_back(declaration);

        Some(children)
    }
}
