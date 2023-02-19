use std::collections::LinkedList;

use antlr_rust::{
    parser::ParserNodeType,
    tree::{
        ParseTreeListener, TerminalNode, ErrorNode
    }, CoerceTo, parser_rule_context::ParserRuleContext
};

use super::generated::javaparserlistener::JavaParserListener;
use super::generated::javaparser::*;
use super::super::inode::*;

pub struct Listener {
    stack: LinkedList<Node>,
}

impl Listener {
    pub fn new() -> Self {
        Listener { stack: LinkedList::new() }
    }

    pub fn get_stack(&mut self) -> &mut LinkedList<Node> {
        &mut self.stack
    }

    fn update_stack<T: Fn(&mut Node)>(&mut self, node_type: NodeType, update_attrs: T) {
        match self.stack.pop_back() {
            Some(mut node) => {
                if node.get_node_type() != node_type {
                    panic!("[ERROR] invalid node type. expected: {:?}, actual: {:?}", node_type, node.get_node_type())
                }

                update_attrs(&mut node);

                match self.stack.back_mut() {
                    Some(parent) => {
                        parent.get_members().push_back(node);
                    },
                    None => panic!("[ERROR] invalid status. parent node of {:?} not found.", node_type)
                }
            },
            None => panic!("[ERROR] invalid status. {:?} node not found", node_type)
        };
    }
}

impl<'input, 'a, Node: ParserNodeType<'input>> ParseTreeListener<'input, Node> for Listener {
    /// Called when parser creates terminal node
    fn visit_terminal(&mut self, _node: &TerminalNode<'input, Node>) {
        // println!("visit_terminal: {:?}", _node);
    }
    /// Called when parser creates error node
    fn visit_error_node(&mut self, _node: &ErrorNode<'input, Node>) {
        // println!("visit_error_node: {:?}", _node);
    }
    /// Called when parser enters any rule node
    fn enter_every_rule(&mut self, _ctx: &Node::Type) {
        // println!("enter_every_rule: {:?}", _ctx);
    }
    /// Called when parser exits any rule node
    fn exit_every_rule(&mut self, _ctx: &Node::Type) {
        // println!("exit_every_rule: {:?}", _ctx);
    }
}

impl<'input> JavaParserListener<'input> for Listener {
    /**
     * Enter a parse tree produced by {@link JavaParser#compilationUnit}.
     * @param ctx the parse tree
     */
    fn enter_compilationUnit(&mut self, _ctx: &CompilationUnitContext<'input>) {
        // println!("enter_compilationUnit: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#compilationUnit}.
     * @param ctx the parse tree
     */
    fn exit_compilationUnit(&mut self, _ctx: &CompilationUnitContext<'input>) {
        // println!("exit_compilationUnit: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#packageDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_packageDeclaration(&mut self, _ctx: &PackageDeclarationContext<'input>) {
        self.stack.push_back(Node::new(NodeType::Package))
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#packageDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_packageDeclaration(&mut self, _ctx: &PackageDeclarationContext<'input>) {
        match _ctx.qualifiedName() {
            Some(qn) => {
                let mut ids = Vec::new();
                for id in qn.identifier_all() {
                    ids.push(format!("{:?}", id.IDENTIFIER().unwrap()));
                }

                self.update_stack(NodeType::Package, |node| {
                    node.set_attr("name", ids.join(".").as_str());
                });
            },
            None => {}
        }
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#importDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_importDeclaration(&mut self, _ctx: &ImportDeclarationContext<'input>) {
        self.stack.push_back(Node::new(NodeType::Import))
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#importDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_importDeclaration(&mut self, _ctx: &ImportDeclarationContext<'input>) {
        match _ctx.qualifiedName() {
            Some(qn) => {
                let mut ids = Vec::new();
                for id in qn.identifier_all() {
                    ids.push(format!("{:?}", id.IDENTIFIER().unwrap()));
                }

                self.update_stack(NodeType::Import, |node| {
                    node.set_attr("name", ids.join(".").as_str());
                });
            },
            None => {}
        }
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#typeDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_typeDeclaration(&mut self, _ctx: &TypeDeclarationContext<'input>) {
        println!("enter_typeDeclaration: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#typeDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_typeDeclaration(&mut self, _ctx: &TypeDeclarationContext<'input>) {
        println!("exit_typeDeclaration: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#modifier}.
     * @param ctx the parse tree
     */
    fn enter_modifier(&mut self, _ctx: &ModifierContext<'input>) {
        println!("enter_modifier: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#modifier}.
     * @param ctx the parse tree
     */
    fn exit_modifier(&mut self, _ctx: &ModifierContext<'input>) {
        println!("exit_modifier: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#classOrInterfaceModifier}.
     * @param ctx the parse tree
     */
    fn enter_classOrInterfaceModifier(&mut self, _ctx: &ClassOrInterfaceModifierContext<'input>) {
        println!("enter_classOrInterfaceModifier: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#classOrInterfaceModifier}.
     * @param ctx the parse tree
     */
    fn exit_classOrInterfaceModifier(&mut self, _ctx: &ClassOrInterfaceModifierContext<'input>) {
        println!("exit_classOrInterfaceModifier: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#variableModifier}.
     * @param ctx the parse tree
     */
    fn enter_variableModifier(&mut self, _ctx: &VariableModifierContext<'input>) {
        println!("enter_variableModifier: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#variableModifier}.
     * @param ctx the parse tree
     */
    fn exit_variableModifier(&mut self, _ctx: &VariableModifierContext<'input>) {
        println!("exit_variableModifier: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#classDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_classDeclaration(&mut self, _ctx: &ClassDeclarationContext<'input>) {
        println!("enter_classDeclaration: {:?}", _ctx.CLASS());
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#classDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_classDeclaration(&mut self, _ctx: &ClassDeclarationContext<'input>) {
        println!("exit_classDeclaration: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#typeParameters}.
     * @param ctx the parse tree
     */
    fn enter_typeParameters(&mut self, _ctx: &TypeParametersContext<'input>) {
        println!("enter_typeParameters: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#typeParameters}.
     * @param ctx the parse tree
     */
    fn exit_typeParameters(&mut self, _ctx: &TypeParametersContext<'input>) {
        println!("exit_typeParameters: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#typeParameter}.
     * @param ctx the parse tree
     */
    fn enter_typeParameter(&mut self, _ctx: &TypeParameterContext<'input>) {
        println!("enter_typeParameter: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#typeParameter}.
     * @param ctx the parse tree
     */
    fn exit_typeParameter(&mut self, _ctx: &TypeParameterContext<'input>) {
        println!("exit_typeParameter: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#typeBound}.
     * @param ctx the parse tree
     */
    fn enter_typeBound(&mut self, _ctx: &TypeBoundContext<'input>) {
        println!("enter_typeBound: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#typeBound}.
     * @param ctx the parse tree
     */
    fn exit_typeBound(&mut self, _ctx: &TypeBoundContext<'input>) {
        println!("exit_typeBound: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#enumDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_enumDeclaration(&mut self, _ctx: &EnumDeclarationContext<'input>) {
        println!("enter_enumDeclaration: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#enumDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_enumDeclaration(&mut self, _ctx: &EnumDeclarationContext<'input>) {
        println!("exit_enumDeclaration: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#enumConstants}.
     * @param ctx the parse tree
     */
    fn enter_enumConstants(&mut self, _ctx: &EnumConstantsContext<'input>) {
        println!("enter_enumConstants: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#enumConstants}.
     * @param ctx the parse tree
     */
    fn exit_enumConstants(&mut self, _ctx: &EnumConstantsContext<'input>) {
        println!("exit_enumConstants: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#enumConstant}.
     * @param ctx the parse tree
     */
    fn enter_enumConstant(&mut self, _ctx: &EnumConstantContext<'input>) {
        println!("enter_enumConstant: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#enumConstant}.
     * @param ctx the parse tree
     */
    fn exit_enumConstant(&mut self, _ctx: &EnumConstantContext<'input>) {
        println!("exit_enumConstant: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#enumBodyDeclarations}.
     * @param ctx the parse tree
     */
    fn enter_enumBodyDeclarations(&mut self, _ctx: &EnumBodyDeclarationsContext<'input>) {
        println!("enter_enumBodyDeclarations: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#enumBodyDeclarations}.
     * @param ctx the parse tree
     */
    fn exit_enumBodyDeclarations(&mut self, _ctx: &EnumBodyDeclarationsContext<'input>) {
        println!("exit_enumBodyDeclarations: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#interfaceDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_interfaceDeclaration(&mut self, _ctx: &InterfaceDeclarationContext<'input>) {
        println!("enter_interfaceDeclaration: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#interfaceDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_interfaceDeclaration(&mut self, _ctx: &InterfaceDeclarationContext<'input>) {
        println!("exit_interfaceDeclaration: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#classBody}.
     * @param ctx the parse tree
     */
    fn enter_classBody(&mut self, _ctx: &ClassBodyContext<'input>) {
        println!("enter_classBody: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#classBody}.
     * @param ctx the parse tree
     */
    fn exit_classBody(&mut self, _ctx: &ClassBodyContext<'input>) {
        println!("exit_classBody: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#interfaceBody}.
     * @param ctx the parse tree
     */
    fn enter_interfaceBody(&mut self, _ctx: &InterfaceBodyContext<'input>) {
        println!("enter_interfaceBody: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#interfaceBody}.
     * @param ctx the parse tree
     */
    fn exit_interfaceBody(&mut self, _ctx: &InterfaceBodyContext<'input>) {
        println!("exit_interfaceBody: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#classBodyDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_classBodyDeclaration(&mut self, _ctx: &ClassBodyDeclarationContext<'input>) {
        println!("enter_classBodyDeclaration: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#classBodyDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_classBodyDeclaration(&mut self, _ctx: &ClassBodyDeclarationContext<'input>) {
        println!("exit_classBodyDeclaration: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#memberDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_memberDeclaration(&mut self, _ctx: &MemberDeclarationContext<'input>) {
        println!("enter_memberDeclaration: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#memberDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_memberDeclaration(&mut self, _ctx: &MemberDeclarationContext<'input>) {
        println!("exit_memberDeclaration: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#methodDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_methodDeclaration(&mut self, _ctx: &MethodDeclarationContext<'input>) {
        println!("enter_methodDeclaration: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#methodDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_methodDeclaration(&mut self, _ctx: &MethodDeclarationContext<'input>) {
        println!("exit_methodDeclaration: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#methodBody}.
     * @param ctx the parse tree
     */
    fn enter_methodBody(&mut self, _ctx: &MethodBodyContext<'input>) {
        println!("enter_methodBody: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#methodBody}.
     * @param ctx the parse tree
     */
    fn exit_methodBody(&mut self, _ctx: &MethodBodyContext<'input>) {
        println!("exit_methodBody: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#typeTypeOrVoid}.
     * @param ctx the parse tree
     */
    fn enter_typeTypeOrVoid(&mut self, _ctx: &TypeTypeOrVoidContext<'input>) {
        println!("enter_typeTypeOrVoid: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#typeTypeOrVoid}.
     * @param ctx the parse tree
     */
    fn exit_typeTypeOrVoid(&mut self, _ctx: &TypeTypeOrVoidContext<'input>) {
        println!("exit_typeTypeOrVoid: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#genericMethodDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_genericMethodDeclaration(&mut self, _ctx: &GenericMethodDeclarationContext<'input>) {
        println!("enter_genericMethodDeclaration: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#genericMethodDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_genericMethodDeclaration(&mut self, _ctx: &GenericMethodDeclarationContext<'input>) {
        println!("exit_genericMethodDeclaration: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#genericConstructorDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_genericConstructorDeclaration(&mut self, _ctx: &GenericConstructorDeclarationContext<'input>) {
        println!("enter_genericConstructorDeclaration: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#genericConstructorDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_genericConstructorDeclaration(&mut self, _ctx: &GenericConstructorDeclarationContext<'input>) {
        println!("exit_genericConstructorDeclaration: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#constructorDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_constructorDeclaration(&mut self, _ctx: &ConstructorDeclarationContext<'input>) {
        println!("enter_constructorDeclaration: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#constructorDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_constructorDeclaration(&mut self, _ctx: &ConstructorDeclarationContext<'input>) {
        println!("exit_constructorDeclaration: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#compactConstructorDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_compactConstructorDeclaration(&mut self, _ctx: &CompactConstructorDeclarationContext<'input>) {
        println!("enter_compactConstructorDeclaration: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#compactConstructorDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_compactConstructorDeclaration(&mut self, _ctx: &CompactConstructorDeclarationContext<'input>) {
        println!("exit_compactConstructorDeclaration: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#fieldDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_fieldDeclaration(&mut self, _ctx: &FieldDeclarationContext<'input>) {
        println!("enter_fieldDeclaration: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#fieldDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_fieldDeclaration(&mut self, _ctx: &FieldDeclarationContext<'input>) {
        println!("exit_fieldDeclaration: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#interfaceBodyDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_interfaceBodyDeclaration(&mut self, _ctx: &InterfaceBodyDeclarationContext<'input>) {
        println!("enter_interfaceBodyDeclaration: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#interfaceBodyDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_interfaceBodyDeclaration(&mut self, _ctx: &InterfaceBodyDeclarationContext<'input>) {
        println!("exit_interfaceBodyDeclaration: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#interfaceMemberDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_interfaceMemberDeclaration(&mut self, _ctx: &InterfaceMemberDeclarationContext<'input>) {
        println!("enter_interfaceMemberDeclaration: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#interfaceMemberDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_interfaceMemberDeclaration(&mut self, _ctx: &InterfaceMemberDeclarationContext<'input>) {
        println!("exit_interfaceMemberDeclaration: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#constDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_constDeclaration(&mut self, _ctx: &ConstDeclarationContext<'input>) {
        println!("enter_constDeclaration: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#constDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_constDeclaration(&mut self, _ctx: &ConstDeclarationContext<'input>) {
        println!("exit_constDeclaration: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#constantDeclarator}.
     * @param ctx the parse tree
     */
    fn enter_constantDeclarator(&mut self, _ctx: &ConstantDeclaratorContext<'input>) {
        println!("enter_constantDeclarator: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#constantDeclarator}.
     * @param ctx the parse tree
     */
    fn exit_constantDeclarator(&mut self, _ctx: &ConstantDeclaratorContext<'input>) {
        println!("exit_constantDeclarator: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#interfaceMethodDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_interfaceMethodDeclaration(&mut self, _ctx: &InterfaceMethodDeclarationContext<'input>) {
        println!("enter_interfaceMethodDeclaration: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#interfaceMethodDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_interfaceMethodDeclaration(&mut self, _ctx: &InterfaceMethodDeclarationContext<'input>) {
        println!("exit_interfaceMethodDeclaration: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#interfaceMethodModifier}.
     * @param ctx the parse tree
     */
    fn enter_interfaceMethodModifier(&mut self, _ctx: &InterfaceMethodModifierContext<'input>) {
        println!("enter_interfaceMethodModifier: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#interfaceMethodModifier}.
     * @param ctx the parse tree
     */
    fn exit_interfaceMethodModifier(&mut self, _ctx: &InterfaceMethodModifierContext<'input>) {
        println!("exit_interfaceMethodModifier: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#genericInterfaceMethodDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_genericInterfaceMethodDeclaration(&mut self, _ctx: &GenericInterfaceMethodDeclarationContext<'input>) {
        println!("enter_genericInterfaceMethodDeclaration: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#genericInterfaceMethodDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_genericInterfaceMethodDeclaration(&mut self, _ctx: &GenericInterfaceMethodDeclarationContext<'input>) {
        println!("exit_genericInterfaceMethodDeclaration: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#interfaceCommonBodyDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_interfaceCommonBodyDeclaration(&mut self, _ctx: &InterfaceCommonBodyDeclarationContext<'input>) {
        println!("enter_interfaceCommonBodyDeclaration: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#interfaceCommonBodyDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_interfaceCommonBodyDeclaration(&mut self, _ctx: &InterfaceCommonBodyDeclarationContext<'input>) {
        println!("exit_interfaceCommonBodyDeclaration: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#variableDeclarators}.
     * @param ctx the parse tree
     */
    fn enter_variableDeclarators(&mut self, _ctx: &VariableDeclaratorsContext<'input>) {
        println!("enter_variableDeclarators: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#variableDeclarators}.
     * @param ctx the parse tree
     */
    fn exit_variableDeclarators(&mut self, _ctx: &VariableDeclaratorsContext<'input>) {
        println!("exit_variableDeclarators: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#variableDeclarator}.
     * @param ctx the parse tree
     */
    fn enter_variableDeclarator(&mut self, _ctx: &VariableDeclaratorContext<'input>) {
        println!("enter_variableDeclarator: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#variableDeclarator}.
     * @param ctx the parse tree
     */
    fn exit_variableDeclarator(&mut self, _ctx: &VariableDeclaratorContext<'input>) {
        println!("exit_variableDeclarator: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#variableDeclaratorId}.
     * @param ctx the parse tree
     */
    fn enter_variableDeclaratorId(&mut self, _ctx: &VariableDeclaratorIdContext<'input>) {
        println!("enter_variableDeclaratorId: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#variableDeclaratorId}.
     * @param ctx the parse tree
     */
    fn exit_variableDeclaratorId(&mut self, _ctx: &VariableDeclaratorIdContext<'input>) {
        println!("exit_variableDeclaratorId: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#variableInitializer}.
     * @param ctx the parse tree
     */
    fn enter_variableInitializer(&mut self, _ctx: &VariableInitializerContext<'input>) {
        println!("enter_variableInitializer: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#variableInitializer}.
     * @param ctx the parse tree
     */
    fn exit_variableInitializer(&mut self, _ctx: &VariableInitializerContext<'input>) {
        println!("exit_variableInitializer: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#arrayInitializer}.
     * @param ctx the parse tree
     */
    fn enter_arrayInitializer(&mut self, _ctx: &ArrayInitializerContext<'input>) {
        println!("enter_arrayInitializer: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#arrayInitializer}.
     * @param ctx the parse tree
     */
    fn exit_arrayInitializer(&mut self, _ctx: &ArrayInitializerContext<'input>) {
        println!("exit_arrayInitializer: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#classOrInterfaceType}.
     * @param ctx the parse tree
     */
    fn enter_classOrInterfaceType(&mut self, _ctx: &ClassOrInterfaceTypeContext<'input>) {
        println!("enter_classOrInterfaceType: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#classOrInterfaceType}.
     * @param ctx the parse tree
     */
    fn exit_classOrInterfaceType(&mut self, _ctx: &ClassOrInterfaceTypeContext<'input>) {
        println!("exit_classOrInterfaceType: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#typeArgument}.
     * @param ctx the parse tree
     */
    fn enter_typeArgument(&mut self, _ctx: &TypeArgumentContext<'input>) {
        println!("enter_typeArgument: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#typeArgument}.
     * @param ctx the parse tree
     */
    fn exit_typeArgument(&mut self, _ctx: &TypeArgumentContext<'input>) {
        println!("exit_typeArgument: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#qualifiedNameList}.
     * @param ctx the parse tree
     */
    fn enter_qualifiedNameList(&mut self, _ctx: &QualifiedNameListContext<'input>) {
        println!("enter_qualifiedNameList: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#qualifiedNameList}.
     * @param ctx the parse tree
     */
    fn exit_qualifiedNameList(&mut self, _ctx: &QualifiedNameListContext<'input>) {
        println!("exit_qualifiedNameList: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#formalParameters}.
     * @param ctx the parse tree
     */
    fn enter_formalParameters(&mut self, _ctx: &FormalParametersContext<'input>) {
        println!("enter_formalParameters: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#formalParameters}.
     * @param ctx the parse tree
     */
    fn exit_formalParameters(&mut self, _ctx: &FormalParametersContext<'input>) {
        println!("exit_formalParameters: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#receiverParameter}.
     * @param ctx the parse tree
     */
    fn enter_receiverParameter(&mut self, _ctx: &ReceiverParameterContext<'input>) {
        println!("enter_receiverParameter: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#receiverParameter}.
     * @param ctx the parse tree
     */
    fn exit_receiverParameter(&mut self, _ctx: &ReceiverParameterContext<'input>) {
        println!("exit_receiverParameter: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#formalParameterList}.
     * @param ctx the parse tree
     */
    fn enter_formalParameterList(&mut self, _ctx: &FormalParameterListContext<'input>) {
        println!("enter_formalParameterList: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#formalParameterList}.
     * @param ctx the parse tree
     */
    fn exit_formalParameterList(&mut self, _ctx: &FormalParameterListContext<'input>) {
        println!("exit_formalParameterList: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#formalParameter}.
     * @param ctx the parse tree
     */
    fn enter_formalParameter(&mut self, _ctx: &FormalParameterContext<'input>) {
        println!("enter_formalParameter: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#formalParameter}.
     * @param ctx the parse tree
     */
    fn exit_formalParameter(&mut self, _ctx: &FormalParameterContext<'input>) {
        println!("exit_formalParameter: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#lastFormalParameter}.
     * @param ctx the parse tree
     */
    fn enter_lastFormalParameter(&mut self, _ctx: &LastFormalParameterContext<'input>) {
        println!("enter_lastFormalParameter: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#lastFormalParameter}.
     * @param ctx the parse tree
     */
    fn exit_lastFormalParameter(&mut self, _ctx: &LastFormalParameterContext<'input>) {
        println!("exit_lastFormalParameter: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#lambdaLVTIList}.
     * @param ctx the parse tree
     */
    fn enter_lambdaLVTIList(&mut self, _ctx: &LambdaLVTIListContext<'input>) {
        println!("enter_lambdaLVTIList: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#lambdaLVTIList}.
     * @param ctx the parse tree
     */
    fn exit_lambdaLVTIList(&mut self, _ctx: &LambdaLVTIListContext<'input>) {
        println!("exit_lambdaLVTIList: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#lambdaLVTIParameter}.
     * @param ctx the parse tree
     */
    fn enter_lambdaLVTIParameter(&mut self, _ctx: &LambdaLVTIParameterContext<'input>) {
        println!("enter_lambdaLVTIParameter: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#lambdaLVTIParameter}.
     * @param ctx the parse tree
     */
    fn exit_lambdaLVTIParameter(&mut self, _ctx: &LambdaLVTIParameterContext<'input>) {
        println!("exit_lambdaLVTIParameter: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#qualifiedName}.
     * @param ctx the parse tree
     */
    fn enter_qualifiedName(&mut self, _ctx: &QualifiedNameContext<'input>) {
        println!("enter_qualifiedName: {:?}", _ctx.identifier_all().as_slice());
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#qualifiedName}.
     * @param ctx the parse tree
     */
    fn exit_qualifiedName(&mut self, _ctx: &QualifiedNameContext<'input>) {
        println!("exit_qualifiedName: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#literal}.
     * @param ctx the parse tree
     */
    fn enter_literal(&mut self, _ctx: &LiteralContext<'input>) {
        println!("enter_literal: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#literal}.
     * @param ctx the parse tree
     */
    fn exit_literal(&mut self, _ctx: &LiteralContext<'input>) {
        println!("exit_literal: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#integerLiteral}.
     * @param ctx the parse tree
     */
    fn enter_integerLiteral(&mut self, _ctx: &IntegerLiteralContext<'input>) {
        println!("enter_integerLiteral: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#integerLiteral}.
     * @param ctx the parse tree
     */
    fn exit_integerLiteral(&mut self, _ctx: &IntegerLiteralContext<'input>) {
        println!("exit_integerLiteral: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#floatLiteral}.
     * @param ctx the parse tree
     */
    fn enter_floatLiteral(&mut self, _ctx: &FloatLiteralContext<'input>) {
        println!("enter_floatLiteral: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#floatLiteral}.
     * @param ctx the parse tree
     */
    fn exit_floatLiteral(&mut self, _ctx: &FloatLiteralContext<'input>) {
        println!("exit_floatLiteral: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#altAnnotationQualifiedName}.
     * @param ctx the parse tree
     */
    fn enter_altAnnotationQualifiedName(&mut self, _ctx: &AltAnnotationQualifiedNameContext<'input>) {
        println!("enter_altAnnotationQualifiedName: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#altAnnotationQualifiedName}.
     * @param ctx the parse tree
     */
    fn exit_altAnnotationQualifiedName(&mut self, _ctx: &AltAnnotationQualifiedNameContext<'input>) {
        println!("exit_altAnnotationQualifiedName: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#annotation}.
     * @param ctx the parse tree
     */
    fn enter_annotation(&mut self, _ctx: &AnnotationContext<'input>) {
        println!("enter_annotation: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#annotation}.
     * @param ctx the parse tree
     */
    fn exit_annotation(&mut self, _ctx: &AnnotationContext<'input>) {
        println!("exit_annotation: {:?}", _ctx.elementValuePairs().into_iter().count());
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#elementValuePairs}.
     * @param ctx the parse tree
     */
    fn enter_elementValuePairs(&mut self, _ctx: &ElementValuePairsContext<'input>) {
        println!("enter_elementValuePairs: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#elementValuePairs}.
     * @param ctx the parse tree
     */
    fn exit_elementValuePairs(&mut self, _ctx: &ElementValuePairsContext<'input>) {
        println!("exit_elementValuePairs: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#elementValuePair}.
     * @param ctx the parse tree
     */
    fn enter_elementValuePair(&mut self, _ctx: &ElementValuePairContext<'input>) {
        println!("enter_elementValuePair: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#elementValuePair}.
     * @param ctx the parse tree
     */
    fn exit_elementValuePair(&mut self, _ctx: &ElementValuePairContext<'input>) {
        println!("exit_elementValuePair: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#elementValue}.
     * @param ctx the parse tree
     */
    fn enter_elementValue(&mut self, _ctx: &ElementValueContext<'input>) {
        println!("enter_elementValue: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#elementValue}.
     * @param ctx the parse tree
     */
    fn exit_elementValue(&mut self, _ctx: &ElementValueContext<'input>) {
        println!("exit_elementValue: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#elementValueArrayInitializer}.
     * @param ctx the parse tree
     */
    fn enter_elementValueArrayInitializer(&mut self, _ctx: &ElementValueArrayInitializerContext<'input>) {
        println!("enter_elementValueArrayInitializer: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#elementValueArrayInitializer}.
     * @param ctx the parse tree
     */
    fn exit_elementValueArrayInitializer(&mut self, _ctx: &ElementValueArrayInitializerContext<'input>) {
        println!("exit_elementValueArrayInitializer: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#annotationTypeDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_annotationTypeDeclaration(&mut self, _ctx: &AnnotationTypeDeclarationContext<'input>) {
        println!("enter_annotationTypeDeclaration: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#annotationTypeDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_annotationTypeDeclaration(&mut self, _ctx: &AnnotationTypeDeclarationContext<'input>) {
        println!("exit_annotationTypeDeclaration: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#annotationTypeBody}.
     * @param ctx the parse tree
     */
    fn enter_annotationTypeBody(&mut self, _ctx: &AnnotationTypeBodyContext<'input>) {
        println!("enter_annotationTypeBody: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#annotationTypeBody}.
     * @param ctx the parse tree
     */
    fn exit_annotationTypeBody(&mut self, _ctx: &AnnotationTypeBodyContext<'input>) {
        println!("exit_annotationTypeBody: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#annotationTypeElementDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_annotationTypeElementDeclaration(&mut self, _ctx: &AnnotationTypeElementDeclarationContext<'input>) {
        println!("enter_annotationTypeElementDeclaration: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#annotationTypeElementDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_annotationTypeElementDeclaration(&mut self, _ctx: &AnnotationTypeElementDeclarationContext<'input>) {
        println!("exit_annotationTypeElementDeclaration: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#annotationTypeElementRest}.
     * @param ctx the parse tree
     */
    fn enter_annotationTypeElementRest(&mut self, _ctx: &AnnotationTypeElementRestContext<'input>) {
        println!("enter_annotationTypeElementRest: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#annotationTypeElementRest}.
     * @param ctx the parse tree
     */
    fn exit_annotationTypeElementRest(&mut self, _ctx: &AnnotationTypeElementRestContext<'input>) {
        println!("exit_annotationTypeElementRest: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#annotationMethodOrConstantRest}.
     * @param ctx the parse tree
     */
    fn enter_annotationMethodOrConstantRest(&mut self, _ctx: &AnnotationMethodOrConstantRestContext<'input>) {
        println!("enter_annotationMethodOrConstantRest: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#annotationMethodOrConstantRest}.
     * @param ctx the parse tree
     */
    fn exit_annotationMethodOrConstantRest(&mut self, _ctx: &AnnotationMethodOrConstantRestContext<'input>) {
        println!("exit_annotationMethodOrConstantRest: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#annotationMethodRest}.
     * @param ctx the parse tree
     */
    fn enter_annotationMethodRest(&mut self, _ctx: &AnnotationMethodRestContext<'input>) {
        println!("enter_annotationMethodRest: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#annotationMethodRest}.
     * @param ctx the parse tree
     */
    fn exit_annotationMethodRest(&mut self, _ctx: &AnnotationMethodRestContext<'input>) {
        println!("exit_annotationMethodRest: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#annotationConstantRest}.
     * @param ctx the parse tree
     */
    fn enter_annotationConstantRest(&mut self, _ctx: &AnnotationConstantRestContext<'input>) {
        println!("enter_annotationConstantRest: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#annotationConstantRest}.
     * @param ctx the parse tree
     */
    fn exit_annotationConstantRest(&mut self, _ctx: &AnnotationConstantRestContext<'input>) {
        println!("exit_annotationConstantRest: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#defaultValue}.
     * @param ctx the parse tree
     */
    fn enter_defaultValue(&mut self, _ctx: &DefaultValueContext<'input>) {
        println!("enter_defaultValue: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#defaultValue}.
     * @param ctx the parse tree
     */
    fn exit_defaultValue(&mut self, _ctx: &DefaultValueContext<'input>) {
        println!("exit_defaultValue: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#moduleDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_moduleDeclaration(&mut self, _ctx: &ModuleDeclarationContext<'input>) {
        println!("enter_moduleDeclaration: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#moduleDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_moduleDeclaration(&mut self, _ctx: &ModuleDeclarationContext<'input>) {
        println!("exit_moduleDeclaration: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#moduleBody}.
     * @param ctx the parse tree
     */
    fn enter_moduleBody(&mut self, _ctx: &ModuleBodyContext<'input>) {
        println!("enter_moduleBody: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#moduleBody}.
     * @param ctx the parse tree
     */
    fn exit_moduleBody(&mut self, _ctx: &ModuleBodyContext<'input>) {
        println!("exit_moduleBody: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#moduleDirective}.
     * @param ctx the parse tree
     */
    fn enter_moduleDirective(&mut self, _ctx: &ModuleDirectiveContext<'input>) {
        println!("enter_moduleDirective: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#moduleDirective}.
     * @param ctx the parse tree
     */
    fn exit_moduleDirective(&mut self, _ctx: &ModuleDirectiveContext<'input>) {
        println!("exit_moduleDirective: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#requiresModifier}.
     * @param ctx the parse tree
     */
    fn enter_requiresModifier(&mut self, _ctx: &RequiresModifierContext<'input>) {
        println!("enter_requiresModifier: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#requiresModifier}.
     * @param ctx the parse tree
     */
    fn exit_requiresModifier(&mut self, _ctx: &RequiresModifierContext<'input>) {
        println!("exit_requiresModifier: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#recordDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_recordDeclaration(&mut self, _ctx: &RecordDeclarationContext<'input>) {
        println!("enter_recordDeclaration: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#recordDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_recordDeclaration(&mut self, _ctx: &RecordDeclarationContext<'input>) {
        println!("exit_recordDeclaration: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#recordHeader}.
     * @param ctx the parse tree
     */
    fn enter_recordHeader(&mut self, _ctx: &RecordHeaderContext<'input>) {
        println!("enter_recordHeader: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#recordHeader}.
     * @param ctx the parse tree
     */
    fn exit_recordHeader(&mut self, _ctx: &RecordHeaderContext<'input>) {
        println!("exit_recordHeader: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#recordComponentList}.
     * @param ctx the parse tree
     */
    fn enter_recordComponentList(&mut self, _ctx: &RecordComponentListContext<'input>) {
        println!("enter_recordComponentList: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#recordComponentList}.
     * @param ctx the parse tree
     */
    fn exit_recordComponentList(&mut self, _ctx: &RecordComponentListContext<'input>) {
        println!("exit_recordComponentList: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#recordComponent}.
     * @param ctx the parse tree
     */
    fn enter_recordComponent(&mut self, _ctx: &RecordComponentContext<'input>) {
        println!("enter_recordComponent: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#recordComponent}.
     * @param ctx the parse tree
     */
    fn exit_recordComponent(&mut self, _ctx: &RecordComponentContext<'input>) {
        println!("exit_recordComponent: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#recordBody}.
     * @param ctx the parse tree
     */
    fn enter_recordBody(&mut self, _ctx: &RecordBodyContext<'input>) {
        println!("enter_recordBody: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#recordBody}.
     * @param ctx the parse tree
     */
    fn exit_recordBody(&mut self, _ctx: &RecordBodyContext<'input>) {
        println!("exit_recordBody: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#block}.
     * @param ctx the parse tree
     */
    fn enter_block(&mut self, _ctx: &BlockContext<'input>) {
        println!("enter_block: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#block}.
     * @param ctx the parse tree
     */
    fn exit_block(&mut self, _ctx: &BlockContext<'input>) {
        println!("exit_block: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#blockStatement}.
     * @param ctx the parse tree
     */
    fn enter_blockStatement(&mut self, _ctx: &BlockStatementContext<'input>) {
        println!("enter_blockStatement: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#blockStatement}.
     * @param ctx the parse tree
     */
    fn exit_blockStatement(&mut self, _ctx: &BlockStatementContext<'input>) {
        println!("exit_blockStatement: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#localVariableDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_localVariableDeclaration(&mut self, _ctx: &LocalVariableDeclarationContext<'input>) {
        println!("enter_localVariableDeclaration: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#localVariableDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_localVariableDeclaration(&mut self, _ctx: &LocalVariableDeclarationContext<'input>) {
        println!("exit_localVariableDeclaration: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#identifier}.
     * @param ctx the parse tree
     */
    fn enter_identifier(&mut self, _ctx: &IdentifierContext<'input>) {
        println!("enter_identifier: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#identifier}.
     * @param ctx the parse tree
     */
    fn exit_identifier(&mut self, _ctx: &IdentifierContext<'input>) {
        println!("exit_identifier: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#typeIdentifier}.
     * @param ctx the parse tree
     */
    fn enter_typeIdentifier(&mut self, _ctx: &TypeIdentifierContext<'input>) {
        println!("enter_typeIdentifier: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#typeIdentifier}.
     * @param ctx the parse tree
     */
    fn exit_typeIdentifier(&mut self, _ctx: &TypeIdentifierContext<'input>) {
        println!("exit_typeIdentifier: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#localTypeDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_localTypeDeclaration(&mut self, _ctx: &LocalTypeDeclarationContext<'input>) {
        println!("enter_localTypeDeclaration: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#localTypeDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_localTypeDeclaration(&mut self, _ctx: &LocalTypeDeclarationContext<'input>) {
        println!("exit_localTypeDeclaration: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#statement}.
     * @param ctx the parse tree
     */
    fn enter_statement(&mut self, _ctx: &StatementContext<'input>) {
        println!("enter_statement: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#statement}.
     * @param ctx the parse tree
     */
    fn exit_statement(&mut self, _ctx: &StatementContext<'input>) {
        println!("exit_statement: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#catchClause}.
     * @param ctx the parse tree
     */
    fn enter_catchClause(&mut self, _ctx: &CatchClauseContext<'input>) {
        println!("enter_catchClause: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#catchClause}.
     * @param ctx the parse tree
     */
    fn exit_catchClause(&mut self, _ctx: &CatchClauseContext<'input>) {
        println!("exit_catchClause: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#catchType}.
     * @param ctx the parse tree
     */
    fn enter_catchType(&mut self, _ctx: &CatchTypeContext<'input>) {
        println!("enter_catchType: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#catchType}.
     * @param ctx the parse tree
     */
    fn exit_catchType(&mut self, _ctx: &CatchTypeContext<'input>) {
        println!("exit_catchType: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#finallyBlock}.
     * @param ctx the parse tree
     */
    fn enter_finallyBlock(&mut self, _ctx: &FinallyBlockContext<'input>) {
        println!("enter_finallyBlock: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#finallyBlock}.
     * @param ctx the parse tree
     */
    fn exit_finallyBlock(&mut self, _ctx: &FinallyBlockContext<'input>) {
        println!("exit_finallyBlock: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#resourceSpecification}.
     * @param ctx the parse tree
     */
    fn enter_resourceSpecification(&mut self, _ctx: &ResourceSpecificationContext<'input>) {
        println!("enter_resourceSpecification: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#resourceSpecification}.
     * @param ctx the parse tree
     */
    fn exit_resourceSpecification(&mut self, _ctx: &ResourceSpecificationContext<'input>) {
        println!("exit_resourceSpecification: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#resources}.
     * @param ctx the parse tree
     */
    fn enter_resources(&mut self, _ctx: &ResourcesContext<'input>) {
        println!("enter_resources: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#resources}.
     * @param ctx the parse tree
     */
    fn exit_resources(&mut self, _ctx: &ResourcesContext<'input>) {
        println!("exit_resources: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#resource}.
     * @param ctx the parse tree
     */
    fn enter_resource(&mut self, _ctx: &ResourceContext<'input>) {
        println!("enter_resource: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#resource}.
     * @param ctx the parse tree
     */
    fn exit_resource(&mut self, _ctx: &ResourceContext<'input>) {
        println!("exit_resource: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#switchBlockStatementGroup}.
     * @param ctx the parse tree
     */
    fn enter_switchBlockStatementGroup(&mut self, _ctx: &SwitchBlockStatementGroupContext<'input>) {
        println!("enter_switchBlockStatementGroup: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#switchBlockStatementGroup}.
     * @param ctx the parse tree
     */
    fn exit_switchBlockStatementGroup(&mut self, _ctx: &SwitchBlockStatementGroupContext<'input>) {
        println!("exit_switchBlockStatementGroup: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#switchLabel}.
     * @param ctx the parse tree
     */
    fn enter_switchLabel(&mut self, _ctx: &SwitchLabelContext<'input>) {
        println!("enter_switchLabel: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#switchLabel}.
     * @param ctx the parse tree
     */
    fn exit_switchLabel(&mut self, _ctx: &SwitchLabelContext<'input>) {
        println!("exit_switchLabel: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#forControl}.
     * @param ctx the parse tree
     */
    fn enter_forControl(&mut self, _ctx: &ForControlContext<'input>) {
        println!("enter_forControl: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#forControl}.
     * @param ctx the parse tree
     */
    fn exit_forControl(&mut self, _ctx: &ForControlContext<'input>) {
        println!("exit_forControl: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#forInit}.
     * @param ctx the parse tree
     */
    fn enter_forInit(&mut self, _ctx: &ForInitContext<'input>) {
        println!("enter_forInit: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#forInit}.
     * @param ctx the parse tree
     */
    fn exit_forInit(&mut self, _ctx: &ForInitContext<'input>) {
        println!("exit_forInit: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#enhancedForControl}.
     * @param ctx the parse tree
     */
    fn enter_enhancedForControl(&mut self, _ctx: &EnhancedForControlContext<'input>) {
        println!("enter_enhancedForControl: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#enhancedForControl}.
     * @param ctx the parse tree
     */
    fn exit_enhancedForControl(&mut self, _ctx: &EnhancedForControlContext<'input>) {
        println!("exit_enhancedForControl: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#parExpression}.
     * @param ctx the parse tree
     */
    fn enter_parExpression(&mut self, _ctx: &ParExpressionContext<'input>) {
        println!("enter_parExpression: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#parExpression}.
     * @param ctx the parse tree
     */
    fn exit_parExpression(&mut self, _ctx: &ParExpressionContext<'input>) {
        println!("exit_parExpression: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#expressionList}.
     * @param ctx the parse tree
     */
    fn enter_expressionList(&mut self, _ctx: &ExpressionListContext<'input>) {
        println!("enter_expressionList: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#expressionList}.
     * @param ctx the parse tree
     */
    fn exit_expressionList(&mut self, _ctx: &ExpressionListContext<'input>) {
        println!("exit_expressionList: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#methodCall}.
     * @param ctx the parse tree
     */
    fn enter_methodCall(&mut self, _ctx: &MethodCallContext<'input>) {
        println!("enter_methodCall: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#methodCall}.
     * @param ctx the parse tree
     */
    fn exit_methodCall(&mut self, _ctx: &MethodCallContext<'input>) {
        println!("exit_methodCall: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#expression}.
     * @param ctx the parse tree
     */
    fn enter_expression(&mut self, _ctx: &ExpressionContext<'input>) {
        println!("enter_expression: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#expression}.
     * @param ctx the parse tree
     */
    fn exit_expression(&mut self, _ctx: &ExpressionContext<'input>) {
        println!("exit_expression: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#pattern}.
     * @param ctx the parse tree
     */
    fn enter_pattern(&mut self, _ctx: &PatternContext<'input>) {
        println!("enter_pattern: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#pattern}.
     * @param ctx the parse tree
     */
    fn exit_pattern(&mut self, _ctx: &PatternContext<'input>) {
        println!("exit_pattern: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#lambdaExpression}.
     * @param ctx the parse tree
     */
    fn enter_lambdaExpression(&mut self, _ctx: &LambdaExpressionContext<'input>) {
        println!("enter_lambdaExpression: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#lambdaExpression}.
     * @param ctx the parse tree
     */
    fn exit_lambdaExpression(&mut self, _ctx: &LambdaExpressionContext<'input>) {
        println!("exit_lambdaExpression: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#lambdaParameters}.
     * @param ctx the parse tree
     */
    fn enter_lambdaParameters(&mut self, _ctx: &LambdaParametersContext<'input>) {
        println!("enter_lambdaParameters: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#lambdaParameters}.
     * @param ctx the parse tree
     */
    fn exit_lambdaParameters(&mut self, _ctx: &LambdaParametersContext<'input>) {
        println!("exit_lambdaParameters: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#lambdaBody}.
     * @param ctx the parse tree
     */
    fn enter_lambdaBody(&mut self, _ctx: &LambdaBodyContext<'input>) {
        println!("enter_lambdaBody: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#lambdaBody}.
     * @param ctx the parse tree
     */
    fn exit_lambdaBody(&mut self, _ctx: &LambdaBodyContext<'input>) {
        println!("exit_lambdaBody: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#primary}.
     * @param ctx the parse tree
     */
    fn enter_primary(&mut self, _ctx: &PrimaryContext<'input>) {
        println!("enter_primary: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#primary}.
     * @param ctx the parse tree
     */
    fn exit_primary(&mut self, _ctx: &PrimaryContext<'input>) {
        println!("exit_primary: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#switchExpression}.
     * @param ctx the parse tree
     */
    fn enter_switchExpression(&mut self, _ctx: &SwitchExpressionContext<'input>) {
        println!("enter_switchExpression: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#switchExpression}.
     * @param ctx the parse tree
     */
    fn exit_switchExpression(&mut self, _ctx: &SwitchExpressionContext<'input>) {
        println!("enter_switchExpression: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#switchLabeledRule}.
     * @param ctx the parse tree
     */
    fn enter_switchLabeledRule(&mut self, _ctx: &SwitchLabeledRuleContext<'input>) {
        println!("enter_switchLabeledRule: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#switchLabeledRule}.
     * @param ctx the parse tree
     */
    fn exit_switchLabeledRule(&mut self, _ctx: &SwitchLabeledRuleContext<'input>) {
        println!("exit_switchLabeledRule: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#guardedPattern}.
     * @param ctx the parse tree
     */
    fn enter_guardedPattern(&mut self, _ctx: &GuardedPatternContext<'input>) {
        println!("enter_guardedPattern: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#guardedPattern}.
     * @param ctx the parse tree
     */
    fn exit_guardedPattern(&mut self, _ctx: &GuardedPatternContext<'input>) {
        println!("exit_guardedPattern: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#switchRuleOutcome}.
     * @param ctx the parse tree
     */
    fn enter_switchRuleOutcome(&mut self, _ctx: &SwitchRuleOutcomeContext<'input>) {
        println!("enter_switchRuleOutcome: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#switchRuleOutcome}.
     * @param ctx the parse tree
     */
    fn exit_switchRuleOutcome(&mut self, _ctx: &SwitchRuleOutcomeContext<'input>) {
        println!("exit_switchRuleOutcome: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#classType}.
     * @param ctx the parse tree
     */
    fn enter_classType(&mut self, _ctx: &ClassTypeContext<'input>) {
        println!("enter_classType: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#classType}.
     * @param ctx the parse tree
     */
    fn exit_classType(&mut self, _ctx: &ClassTypeContext<'input>) {
        println!("exit_classType: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#creator}.
     * @param ctx the parse tree
     */
    fn enter_creator(&mut self, _ctx: &CreatorContext<'input>) {
        println!("enter_creator: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#creator}.
     * @param ctx the parse tree
     */
    fn exit_creator(&mut self, _ctx: &CreatorContext<'input>) {
        println!("exit_creator: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#createdName}.
     * @param ctx the parse tree
     */
    fn enter_createdName(&mut self, _ctx: &CreatedNameContext<'input>) {
        println!("enter_createdName: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#createdName}.
     * @param ctx the parse tree
     */
    fn exit_createdName(&mut self, _ctx: &CreatedNameContext<'input>) {
        println!("exit_createdName: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#innerCreator}.
     * @param ctx the parse tree
     */
    fn enter_innerCreator(&mut self, _ctx: &InnerCreatorContext<'input>) {
        println!("enter_innerCreator: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#innerCreator}.
     * @param ctx the parse tree
     */
    fn exit_innerCreator(&mut self, _ctx: &InnerCreatorContext<'input>) {
        println!("exit_innerCreator: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#arrayCreatorRest}.
     * @param ctx the parse tree
     */
    fn enter_arrayCreatorRest(&mut self, _ctx: &ArrayCreatorRestContext<'input>) {
        println!("enter_arrayCreatorRest: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#arrayCreatorRest}.
     * @param ctx the parse tree
     */
    fn exit_arrayCreatorRest(&mut self, _ctx: &ArrayCreatorRestContext<'input>) {
        println!("exit_arrayCreatorRest: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#classCreatorRest}.
     * @param ctx the parse tree
     */
    fn enter_classCreatorRest(&mut self, _ctx: &ClassCreatorRestContext<'input>) {
        println!("enter_classCreatorRest: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#classCreatorRest}.
     * @param ctx the parse tree
     */
    fn exit_classCreatorRest(&mut self, _ctx: &ClassCreatorRestContext<'input>) {
        println!("exit_classCreatorRest: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#explicitGenericInvocation}.
     * @param ctx the parse tree
     */
    fn enter_explicitGenericInvocation(&mut self, _ctx: &ExplicitGenericInvocationContext<'input>) {
        println!("enter_explicitGenericInvocation: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#explicitGenericInvocation}.
     * @param ctx the parse tree
     */
    fn exit_explicitGenericInvocation(&mut self, _ctx: &ExplicitGenericInvocationContext<'input>) {
        println!("exit_explicitGenericInvocation: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#typeArgumentsOrDiamond}.
     * @param ctx the parse tree
     */
    fn enter_typeArgumentsOrDiamond(&mut self, _ctx: &TypeArgumentsOrDiamondContext<'input>) {
        println!("enter_typeArgumentsOrDiamond: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#typeArgumentsOrDiamond}.
     * @param ctx the parse tree
     */
    fn exit_typeArgumentsOrDiamond(&mut self, _ctx: &TypeArgumentsOrDiamondContext<'input>) {
        println!("exit_typeArgumentsOrDiamond: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#nonWildcardTypeArgumentsOrDiamond}.
     * @param ctx the parse tree
     */
    fn enter_nonWildcardTypeArgumentsOrDiamond(&mut self, _ctx: &NonWildcardTypeArgumentsOrDiamondContext<'input>) {
        println!("enter_nonWildcardTypeArgumentsOrDiamond: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#nonWildcardTypeArgumentsOrDiamond}.
     * @param ctx the parse tree
     */
    fn exit_nonWildcardTypeArgumentsOrDiamond(&mut self, _ctx: &NonWildcardTypeArgumentsOrDiamondContext<'input>) {
        println!("exit_nonWildcardTypeArgumentsOrDiamond: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#nonWildcardTypeArguments}.
     * @param ctx the parse tree
     */
    fn enter_nonWildcardTypeArguments(&mut self, _ctx: &NonWildcardTypeArgumentsContext<'input>) {
        println!("enter_nonWildcardTypeArguments: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#nonWildcardTypeArguments}.
     * @param ctx the parse tree
     */
    fn exit_nonWildcardTypeArguments(&mut self, _ctx: &NonWildcardTypeArgumentsContext<'input>) {
        println!("exit_nonWildcardTypeArguments: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#typeList}.
     * @param ctx the parse tree
     */
    fn enter_typeList(&mut self, _ctx: &TypeListContext<'input>) {
        println!("enter_typeList: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#typeList}.
     * @param ctx the parse tree
     */
    fn exit_typeList(&mut self, _ctx: &TypeListContext<'input>) {
        println!("exit_typeList: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#typeType}.
     * @param ctx the parse tree
     */
    fn enter_typeType(&mut self, _ctx: &TypeTypeContext<'input>) {
        println!("enter_typeType: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#typeType}.
     * @param ctx the parse tree
     */
    fn exit_typeType(&mut self, _ctx: &TypeTypeContext<'input>) {
        println!("exit_typeType: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#primitiveType}.
     * @param ctx the parse tree
     */
    fn enter_primitiveType(&mut self, _ctx: &PrimitiveTypeContext<'input>) {
        println!("enter_primitiveType: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#primitiveType}.
     * @param ctx the parse tree
     */
    fn exit_primitiveType(&mut self, _ctx: &PrimitiveTypeContext<'input>) {
        println!("exit_primitiveType: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#typeArguments}.
     * @param ctx the parse tree
     */
    fn enter_typeArguments(&mut self, _ctx: &TypeArgumentsContext<'input>) {
        println!("enter_typeArguments: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#typeArguments}.
     * @param ctx the parse tree
     */
    fn exit_typeArguments(&mut self, _ctx: &TypeArgumentsContext<'input>) {
        println!("exit_typeArguments: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#superSuffix}.
     * @param ctx the parse tree
     */
    fn enter_superSuffix(&mut self, _ctx: &SuperSuffixContext<'input>) {
        println!("enter_superSuffix: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#superSuffix}.
     * @param ctx the parse tree
     */
    fn exit_superSuffix(&mut self, _ctx: &SuperSuffixContext<'input>) {
        println!("exit_superSuffix: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#explicitGenericInvocationSuffix}.
     * @param ctx the parse tree
     */
    fn enter_explicitGenericInvocationSuffix(&mut self, _ctx: &ExplicitGenericInvocationSuffixContext<'input>) {
        println!("enter_explicitGenericInvocationSuffix: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#explicitGenericInvocationSuffix}.
     * @param ctx the parse tree
     */
    fn exit_explicitGenericInvocationSuffix(&mut self, _ctx: &ExplicitGenericInvocationSuffixContext<'input>) {
        println!("exit_explicitGenericInvocationSuffix: {:?}", _ctx);
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#arguments}.
     * @param ctx the parse tree
     */
    fn enter_arguments(&mut self, _ctx: &ArgumentsContext<'input>) {
        println!("enter_arguments: {:?}", _ctx);
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#arguments}.
     * @param ctx the parse tree
     */
    fn exit_arguments(&mut self, _ctx: &ArgumentsContext<'input>) {
        println!("exit_arguments: {:?}", _ctx);
    }
}