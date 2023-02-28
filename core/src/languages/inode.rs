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

            let ctx_opt = self_rf.clone().attr;
            let child_ctx_opt = child.attr.clone();
            if ctx_opt == child_ctx_opt {
                println!("[Removed] Parent: {} - Child: {}", ctx_opt.unwrap(), child_ctx_opt.unwrap());
                if !child.get_members().is_empty() {
                    valid_children.append(child.get_members_mut());
                }
            } else {
                println!("[Kept] Parent: {} - Child: {}", ctx_opt.unwrap(), child_ctx_opt.unwrap());
                valid_children.push_back(child.clone());
            }
        }

        self_rf.members.clear();
        self_rf.members.append(&mut valid_children);
    }
}
