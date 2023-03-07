use antlr_rust::{
    parser::ParserNodeType,
    tree::{ErrorNode, ParseTree, ParseTreeListener, TerminalNode},
};

use suzaku_extension_sdk::{
    stack::Stack,
    language::inode::INode,
};

use super::{
    generated::{
        javaparserlistener::JavaParserListener,
        javaparser::*
    },
    java_node::JavaNode,
    java_node_type::JavaNodeType
};

pub struct ParserListener {
    stack: Stack<JavaNode>,
}

impl ParserListener {
    pub fn new() -> Self {
        ParserListener {
            stack: Stack::new(),
        }
    }

    pub fn stack(&self) -> &Stack<JavaNode> {
        &&self.stack
    }

    pub fn stack_mut(&mut self) -> &mut Stack<JavaNode> {
        &mut self.stack
    }

    fn update_node_attrs<T: Fn(&mut JavaNode)>(
        &mut self,
        node_type: Option<JavaNodeType>,
        update_attrs: T,
    ) -> Option<&JavaNode> {
        let mut top_node = self
            .stack_mut()
            .top_mut()
            .unwrap_or_else(|| panic!("[ERROR] invalid status. parent node not found."));
        if let Some(expected_node_type) = node_type {
            if top_node.get_node_type() != expected_node_type {
                panic!("[ERROR] invalid node type. expected: {:?}, actual: {:?}", expected_node_type, top_node.get_node_type());
            }
        }
        update_attrs(&mut top_node);
        Some(top_node)
    }

    fn add_to_parent_member(&mut self) {
        let mut poped_node = self
            .stack_mut()
            .pop()
            .unwrap_or_else(|| panic!("[ERROR] invalid status. top node not found."));
        poped_node.reorganize_children();

        match poped_node.get_node_type() {
            JavaNodeType::TypeDeclaration
            | JavaNodeType::MemberDeclaration => {
                assert_eq!(poped_node.get_members().len(), 1);
                poped_node = poped_node.get_members_mut().pop_back().unwrap();
            }
            _ => (),
        };

        let parent = self
            .stack_mut()
            .top_mut()
            .unwrap_or_else(|| panic!("[ERROR] invalid status. parent node not found."));
        parent.get_members_mut().push_back(poped_node);
    }
}

impl<'input, 'a, Node: ParserNodeType<'input>> ParseTreeListener<'input, Node> for ParserListener {
    /// Called when parser creates terminal node
    fn visit_terminal(&mut self, _node: &TerminalNode<'input, Node>) {
        match _node.get_text().as_str() {
            "=" | ">" | "<" | "!" | "~" | "?" | ":" | "==" | "<=" | ">=" | "!=" | "&&" | "||"
            | "++" | "--" | "+" | "-" | "*" | "/" | "&" | "|" | "^" | "%" | "+=" | "-=" | "*="
            | "/=" | "&=" | "|=" | "^=" | "%=" | "<<=" | ">>=" | ">>>=" | "->" | "::" => {
                let mut op_node = JavaNode::new(JavaNodeType::Operator);
                op_node.set_attr(_node.get_text().as_str());
                self.stack_mut()
                    .top_mut()
                    .unwrap()
                    .get_members_mut()
                    .push_back(op_node);
            }
            "static" => match self.stack_mut().top_mut().unwrap().get_node_type() {
                JavaNodeType::ImportDeclaration | JavaNodeType::ClassBodyDeclaration => {
                    if let Some(top_node) = self.stack_mut().top_mut() {
                        let mut modifier_node = JavaNode::new(JavaNodeType::Modifier);
                        modifier_node.set_attr(_node.get_text().as_str());
                        top_node.get_members_mut().push_back(modifier_node);
                    }
                }
                _ => (),
            },
            "throws" | "return" | "if" | "else" => {
                if let Some(top_node) = self.stack_mut().top_mut() {
                    let mut keyword_node = JavaNode::new(JavaNodeType::Keyword);
                    keyword_node.set_attr(_node.get_text().as_str());
                    top_node.get_members_mut().push_back(keyword_node);
                }
            }
            _ => (),
        }
    }
    /// Called when parser creates error node
    fn visit_error_node(&mut self, _node: &ErrorNode<'input, Node>) {}
    /// Called when parser enters any rule node
    fn enter_every_rule(&mut self, _ctx: &Node::Type) {}
    /// Called when parser exits any rule node
    fn exit_every_rule(&mut self, _ctx: &Node::Type) {}
}

impl<'input> JavaParserListener<'input> for ParserListener {
    /**
     * Enter a parse tree produced by {@link JavaParser#compilationUnit}.
     * @param ctx the parse tree
     */
    fn enter_compilationUnit(&mut self, _ctx: &CompilationUnitContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link JavaParser#compilationUnit}.
     * @param ctx the parse tree
     */
    fn exit_compilationUnit(&mut self, _ctx: &CompilationUnitContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link JavaParser#packageDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_packageDeclaration(&mut self, _ctx: &PackageDeclarationContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::PackageDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#packageDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_packageDeclaration(&mut self, _ctx: &PackageDeclarationContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::PackageDeclaration), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#importDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_importDeclaration(&mut self, _ctx: &ImportDeclarationContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::ImportDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#importDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_importDeclaration(&mut self, _ctx: &ImportDeclarationContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::ImportDeclaration), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#typeDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_typeDeclaration(&mut self, _ctx: &TypeDeclarationContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::TypeDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#typeDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_typeDeclaration(&mut self, _ctx: &TypeDeclarationContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::TypeDeclaration), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#modifier}.
     * @param ctx the parse tree
     */
    fn enter_modifier(&mut self, _ctx: &ModifierContext<'input>) {
        self.stack_mut().push(JavaNode::new(JavaNodeType::Modifier));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#modifier}.
     * @param ctx the parse tree
     */
    fn exit_modifier(&mut self, _ctx: &ModifierContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::Modifier), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#classOrInterfaceModifier}.
     * @param ctx the parse tree
     */
    fn enter_classOrInterfaceModifier(&mut self, _ctx: &ClassOrInterfaceModifierContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::ClassOrInterfaceModifier));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#classOrInterfaceModifier}.
     * @param ctx the parse tree
     */
    fn exit_classOrInterfaceModifier(&mut self, _ctx: &ClassOrInterfaceModifierContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::ClassOrInterfaceModifier), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#variableModifier}.
     * @param ctx the parse tree
     */
    fn enter_variableModifier(&mut self, _ctx: &VariableModifierContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::VariableModifier));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#variableModifier}.
     * @param ctx the parse tree
     */
    fn exit_variableModifier(&mut self, _ctx: &VariableModifierContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::VariableModifier), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#classDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_classDeclaration(&mut self, _ctx: &ClassDeclarationContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::ClassDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#classDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_classDeclaration(&mut self, _ctx: &ClassDeclarationContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::ClassDeclaration), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#typeParameters}.
     * @param ctx the parse tree
     */
    fn enter_typeParameters(&mut self, _ctx: &TypeParametersContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::TypeParameters));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#typeParameters}.
     * @param ctx the parse tree
     */
    fn exit_typeParameters(&mut self, _ctx: &TypeParametersContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::TypeParameters), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#typeParameter}.
     * @param ctx the parse tree
     */
    fn enter_typeParameter(&mut self, _ctx: &TypeParameterContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::TypeParameter));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#typeParameter}.
     * @param ctx the parse tree
     */
    fn exit_typeParameter(&mut self, _ctx: &TypeParameterContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::TypeParameter), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#typeBound}.
     * @param ctx the parse tree
     */
    fn enter_typeBound(&mut self, _ctx: &TypeBoundContext<'input>) {
        self.stack_mut().push(JavaNode::new(JavaNodeType::TypeBound));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#typeBound}.
     * @param ctx the parse tree
     */
    fn exit_typeBound(&mut self, _ctx: &TypeBoundContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::TypeBound), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#enumDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_enumDeclaration(&mut self, _ctx: &EnumDeclarationContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::EnumDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#enumDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_enumDeclaration(&mut self, _ctx: &EnumDeclarationContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::EnumDeclaration), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#enumConstants}.
     * @param ctx the parse tree
     */
    fn enter_enumConstants(&mut self, _ctx: &EnumConstantsContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::EnumConstants));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#enumConstants}.
     * @param ctx the parse tree
     */
    fn exit_enumConstants(&mut self, _ctx: &EnumConstantsContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::EnumConstants), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#enumConstant}.
     * @param ctx the parse tree
     */
    fn enter_enumConstant(&mut self, _ctx: &EnumConstantContext<'input>) {
        self.stack_mut().push(JavaNode::new(JavaNodeType::EnumConstant));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#enumConstant}.
     * @param ctx the parse tree
     */
    fn exit_enumConstant(&mut self, _ctx: &EnumConstantContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::EnumConstant), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#enumBodyDeclarations}.
     * @param ctx the parse tree
     */
    fn enter_enumBodyDeclarations(&mut self, _ctx: &EnumBodyDeclarationsContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::EnumBodyDeclarations));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#enumBodyDeclarations}.
     * @param ctx the parse tree
     */
    fn exit_enumBodyDeclarations(&mut self, _ctx: &EnumBodyDeclarationsContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::EnumBodyDeclarations), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#interfaceDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_interfaceDeclaration(&mut self, _ctx: &InterfaceDeclarationContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::InterfaceDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#interfaceDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_interfaceDeclaration(&mut self, _ctx: &InterfaceDeclarationContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::InterfaceDeclaration), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#classBody}.
     * @param ctx the parse tree
     */
    fn enter_classBody(&mut self, _ctx: &ClassBodyContext<'input>) {
        self.stack_mut().push(JavaNode::new(JavaNodeType::ClassBody));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#classBody}.
     * @param ctx the parse tree
     */
    fn exit_classBody(&mut self, _ctx: &ClassBodyContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::ClassBody), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#interfaceBody}.
     * @param ctx the parse tree
     */
    fn enter_interfaceBody(&mut self, _ctx: &InterfaceBodyContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::InterfaceBody));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#interfaceBody}.
     * @param ctx the parse tree
     */
    fn exit_interfaceBody(&mut self, _ctx: &InterfaceBodyContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::InterfaceBody), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#classBodyDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_classBodyDeclaration(&mut self, _ctx: &ClassBodyDeclarationContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::ClassBodyDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#classBodyDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_classBodyDeclaration(&mut self, _ctx: &ClassBodyDeclarationContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::ClassBodyDeclaration), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#memberDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_memberDeclaration(&mut self, _ctx: &MemberDeclarationContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::MemberDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#memberDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_memberDeclaration(&mut self, _ctx: &MemberDeclarationContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::MemberDeclaration), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#methodDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_methodDeclaration(&mut self, _ctx: &MethodDeclarationContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::MethodDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#methodDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_methodDeclaration(&mut self, _ctx: &MethodDeclarationContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::MethodDeclaration), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#methodBody}.
     * @param ctx the parse tree
     */
    fn enter_methodBody(&mut self, _ctx: &MethodBodyContext<'input>) {
        self.stack_mut().push(JavaNode::new(JavaNodeType::MethodBody));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#methodBody}.
     * @param ctx the parse tree
     */
    fn exit_methodBody(&mut self, _ctx: &MethodBodyContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::MethodBody), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#typeTypeOrVoid}.
     * @param ctx the parse tree
     */
    fn enter_typeTypeOrVoid(&mut self, _ctx: &TypeTypeOrVoidContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::TypeTypeOrVoid));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#typeTypeOrVoid}.
     * @param ctx the parse tree
     */
    fn exit_typeTypeOrVoid(&mut self, _ctx: &TypeTypeOrVoidContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::TypeTypeOrVoid), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#genericMethodDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_genericMethodDeclaration(&mut self, _ctx: &GenericMethodDeclarationContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::GenericMethodDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#genericMethodDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_genericMethodDeclaration(&mut self, _ctx: &GenericMethodDeclarationContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::GenericMethodDeclaration), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#genericConstructorDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_genericConstructorDeclaration(
        &mut self,
        _ctx: &GenericConstructorDeclarationContext<'input>,
    ) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::GenericConstructorDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#genericConstructorDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_genericConstructorDeclaration(
        &mut self,
        _ctx: &GenericConstructorDeclarationContext<'input>,
    ) {
        self.update_node_attrs(Some(JavaNodeType::GenericConstructorDeclaration), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#constructorDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_constructorDeclaration(&mut self, _ctx: &ConstructorDeclarationContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::ConstructorDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#constructorDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_constructorDeclaration(&mut self, _ctx: &ConstructorDeclarationContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::ConstructorDeclaration), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#compactConstructorDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_compactConstructorDeclaration(
        &mut self,
        _ctx: &CompactConstructorDeclarationContext<'input>,
    ) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::CompactConstructorDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#compactConstructorDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_compactConstructorDeclaration(
        &mut self,
        _ctx: &CompactConstructorDeclarationContext<'input>,
    ) {
        self.update_node_attrs(Some(JavaNodeType::CompactConstructorDeclaration), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#fieldDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_fieldDeclaration(&mut self, _ctx: &FieldDeclarationContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::FieldDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#fieldDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_fieldDeclaration(&mut self, _ctx: &FieldDeclarationContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::FieldDeclaration), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#interfaceBodyDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_interfaceBodyDeclaration(&mut self, _ctx: &InterfaceBodyDeclarationContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::InterfaceBodyDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#interfaceBodyDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_interfaceBodyDeclaration(&mut self, _ctx: &InterfaceBodyDeclarationContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::InterfaceBodyDeclaration), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#interfaceMemberDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_interfaceMemberDeclaration(
        &mut self,
        _ctx: &InterfaceMemberDeclarationContext<'input>,
    ) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::InterfaceMemberDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#interfaceMemberDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_interfaceMemberDeclaration(
        &mut self,
        _ctx: &InterfaceMemberDeclarationContext<'input>,
    ) {
        self.update_node_attrs(Some(JavaNodeType::InterfaceMemberDeclaration), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#constDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_constDeclaration(&mut self, _ctx: &ConstDeclarationContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::ConstDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#constDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_constDeclaration(&mut self, _ctx: &ConstDeclarationContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::ConstDeclaration), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#constantDeclarator}.
     * @param ctx the parse tree
     */
    fn enter_constantDeclarator(&mut self, _ctx: &ConstantDeclaratorContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::ConstantDeclarator));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#constantDeclarator}.
     * @param ctx the parse tree
     */
    fn exit_constantDeclarator(&mut self, _ctx: &ConstantDeclaratorContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::ConstantDeclarator), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#interfaceMethodDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_interfaceMethodDeclaration(
        &mut self,
        _ctx: &InterfaceMethodDeclarationContext<'input>,
    ) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::InterfaceMethodDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#interfaceMethodDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_interfaceMethodDeclaration(
        &mut self,
        _ctx: &InterfaceMethodDeclarationContext<'input>,
    ) {
        self.update_node_attrs(Some(JavaNodeType::InterfaceMethodDeclaration), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#interfaceMethodModifier}.
     * @param ctx the parse tree
     */
    fn enter_interfaceMethodModifier(&mut self, _ctx: &InterfaceMethodModifierContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::InterfaceMethodModifier));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#interfaceMethodModifier}.
     * @param ctx the parse tree
     */
    fn exit_interfaceMethodModifier(&mut self, _ctx: &InterfaceMethodModifierContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::InterfaceMethodModifier), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#genericInterfaceMethodDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_genericInterfaceMethodDeclaration(
        &mut self,
        _ctx: &GenericInterfaceMethodDeclarationContext<'input>,
    ) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::GenericInterfaceMethodDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#genericInterfaceMethodDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_genericInterfaceMethodDeclaration(
        &mut self,
        _ctx: &GenericInterfaceMethodDeclarationContext<'input>,
    ) {
        self.update_node_attrs(Some(JavaNodeType::GenericInterfaceMethodDeclaration), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#interfaceCommonBodyDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_interfaceCommonBodyDeclaration(
        &mut self,
        _ctx: &InterfaceCommonBodyDeclarationContext<'input>,
    ) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::InterfaceCommonBodyDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#interfaceCommonBodyDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_interfaceCommonBodyDeclaration(
        &mut self,
        _ctx: &InterfaceCommonBodyDeclarationContext<'input>,
    ) {
        self.update_node_attrs(Some(JavaNodeType::InterfaceCommonBodyDeclaration), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#variableDeclarators}.
     * @param ctx the parse tree
     */
    fn enter_variableDeclarators(&mut self, _ctx: &VariableDeclaratorsContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::VariableDeclarators));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#variableDeclarators}.
     * @param ctx the parse tree
     */
    fn exit_variableDeclarators(&mut self, _ctx: &VariableDeclaratorsContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::VariableDeclarators), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#variableDeclarator}.
     * @param ctx the parse tree
     */
    fn enter_variableDeclarator(&mut self, _ctx: &VariableDeclaratorContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::VariableDeclarator));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#variableDeclarator}.
     * @param ctx the parse tree
     */
    fn exit_variableDeclarator(&mut self, _ctx: &VariableDeclaratorContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::VariableDeclarator), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#variableDeclaratorId}.
     * @param ctx the parse tree
     */
    fn enter_variableDeclaratorId(&mut self, _ctx: &VariableDeclaratorIdContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::VariableDeclaratorId));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#variableDeclaratorId}.
     * @param ctx the parse tree
     */
    fn exit_variableDeclaratorId(&mut self, _ctx: &VariableDeclaratorIdContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::VariableDeclaratorId), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#variableInitializer}.
     * @param ctx the parse tree
     */
    fn enter_variableInitializer(&mut self, _ctx: &VariableInitializerContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::VariableInitializer));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#variableInitializer}.
     * @param ctx the parse tree
     */
    fn exit_variableInitializer(&mut self, _ctx: &VariableInitializerContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::VariableInitializer), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#arrayInitializer}.
     * @param ctx the parse tree
     */
    fn enter_arrayInitializer(&mut self, _ctx: &ArrayInitializerContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::ArrayInitializer));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#arrayInitializer}.
     * @param ctx the parse tree
     */
    fn exit_arrayInitializer(&mut self, _ctx: &ArrayInitializerContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::ArrayInitializer), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#classOrInterfaceType}.
     * @param ctx the parse tree
     */
    fn enter_classOrInterfaceType(&mut self, _ctx: &ClassOrInterfaceTypeContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::ClassOrInterfaceType));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#classOrInterfaceType}.
     * @param ctx the parse tree
     */
    fn exit_classOrInterfaceType(&mut self, _ctx: &ClassOrInterfaceTypeContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::ClassOrInterfaceType), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#typeArgument}.
     * @param ctx the parse tree
     */
    fn enter_typeArgument(&mut self, _ctx: &TypeArgumentContext<'input>) {
        self.stack_mut().push(JavaNode::new(JavaNodeType::TypeArgument));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#typeArgument}.
     * @param ctx the parse tree
     */
    fn exit_typeArgument(&mut self, _ctx: &TypeArgumentContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::TypeArgument), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#qualifiedNameList}.
     * @param ctx the parse tree
     */
    fn enter_qualifiedNameList(&mut self, _ctx: &QualifiedNameListContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::QualifiedNameList));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#qualifiedNameList}.
     * @param ctx the parse tree
     */
    fn exit_qualifiedNameList(&mut self, _ctx: &QualifiedNameListContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::QualifiedNameList), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#formalParameters}.
     * @param ctx the parse tree
     */
    fn enter_formalParameters(&mut self, _ctx: &FormalParametersContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::FormalParameters));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#formalParameters}.
     * @param ctx the parse tree
     */
    fn exit_formalParameters(&mut self, _ctx: &FormalParametersContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::FormalParameters), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#receiverParameter}.
     * @param ctx the parse tree
     */
    fn enter_receiverParameter(&mut self, _ctx: &ReceiverParameterContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::ReceiverParameter));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#receiverParameter}.
     * @param ctx the parse tree
     */
    fn exit_receiverParameter(&mut self, _ctx: &ReceiverParameterContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::ReceiverParameter), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#formalParameterList}.
     * @param ctx the parse tree
     */
    fn enter_formalParameterList(&mut self, _ctx: &FormalParameterListContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::FormalParameterList));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#formalParameterList}.
     * @param ctx the parse tree
     */
    fn exit_formalParameterList(&mut self, _ctx: &FormalParameterListContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::FormalParameterList), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#formalParameter}.
     * @param ctx the parse tree
     */
    fn enter_formalParameter(&mut self, _ctx: &FormalParameterContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::FormalParameter));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#formalParameter}.
     * @param ctx the parse tree
     */
    fn exit_formalParameter(&mut self, _ctx: &FormalParameterContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::FormalParameter), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#lastFormalParameter}.
     * @param ctx the parse tree
     */
    fn enter_lastFormalParameter(&mut self, _ctx: &LastFormalParameterContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::LastFormalParameter));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#lastFormalParameter}.
     * @param ctx the parse tree
     */
    fn exit_lastFormalParameter(&mut self, _ctx: &LastFormalParameterContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::LastFormalParameter), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#lambdaLVTIList}.
     * @param ctx the parse tree
     */
    fn enter_lambdaLVTIList(&mut self, _ctx: &LambdaLVTIListContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::LambdaLVTIList));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#lambdaLVTIList}.
     * @param ctx the parse tree
     */
    fn exit_lambdaLVTIList(&mut self, _ctx: &LambdaLVTIListContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::LambdaLVTIList), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#lambdaLVTIParameter}.
     * @param ctx the parse tree
     */
    fn enter_lambdaLVTIParameter(&mut self, _ctx: &LambdaLVTIParameterContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::LambdaLVTIParameter));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#lambdaLVTIParameter}.
     * @param ctx the parse tree
     */
    fn exit_lambdaLVTIParameter(&mut self, _ctx: &LambdaLVTIParameterContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::LambdaLVTIParameter), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#qualifiedName}.
     * @param ctx the parse tree
     */
    fn enter_qualifiedName(&mut self, _ctx: &QualifiedNameContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::QualifiedName));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#qualifiedName}.
     * @param ctx the parse tree
     */
    fn exit_qualifiedName(&mut self, _ctx: &QualifiedNameContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::QualifiedName), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#literal}.
     * @param ctx the parse tree
     */
    fn enter_literal(&mut self, _ctx: &LiteralContext<'input>) {
        self.stack_mut().push(JavaNode::new(JavaNodeType::Literal));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#literal}.
     * @param ctx the parse tree
     */
    fn exit_literal(&mut self, _ctx: &LiteralContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::Literal), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#integerLiteral}.
     * @param ctx the parse tree
     */
    fn enter_integerLiteral(&mut self, _ctx: &IntegerLiteralContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::IntegerLiteral));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#integerLiteral}.
     * @param ctx the parse tree
     */
    fn exit_integerLiteral(&mut self, _ctx: &IntegerLiteralContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::IntegerLiteral), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#floatLiteral}.
     * @param ctx the parse tree
     */
    fn enter_floatLiteral(&mut self, _ctx: &FloatLiteralContext<'input>) {
        self.stack_mut().push(JavaNode::new(JavaNodeType::FloatLiteral));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#floatLiteral}.
     * @param ctx the parse tree
     */
    fn exit_floatLiteral(&mut self, _ctx: &FloatLiteralContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::FloatLiteral), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#altAnnotationQualifiedName}.
     * @param ctx the parse tree
     */
    fn enter_altAnnotationQualifiedName(
        &mut self,
        _ctx: &AltAnnotationQualifiedNameContext<'input>,
    ) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::AltAnnotationQualifiedName));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#altAnnotationQualifiedName}.
     * @param ctx the parse tree
     */
    fn exit_altAnnotationQualifiedName(
        &mut self,
        _ctx: &AltAnnotationQualifiedNameContext<'input>,
    ) {
        self.update_node_attrs(Some(JavaNodeType::AltAnnotationQualifiedName), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#annotation}.
     * @param ctx the parse tree
     */
    fn enter_annotation(&mut self, _ctx: &AnnotationContext<'input>) {
        self.stack_mut().push(JavaNode::new(JavaNodeType::Annotation));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#annotation}.
     * @param ctx the parse tree
     */
    fn exit_annotation(&mut self, _ctx: &AnnotationContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::Annotation), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#elementValuePairs}.
     * @param ctx the parse tree
     */
    fn enter_elementValuePairs(&mut self, _ctx: &ElementValuePairsContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::ElementValuePairs));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#elementValuePairs}.
     * @param ctx the parse tree
     */
    fn exit_elementValuePairs(&mut self, _ctx: &ElementValuePairsContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::ElementValuePairs), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#elementValuePair}.
     * @param ctx the parse tree
     */
    fn enter_elementValuePair(&mut self, _ctx: &ElementValuePairContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::ElementValuePair));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#elementValuePair}.
     * @param ctx the parse tree
     */
    fn exit_elementValuePair(&mut self, _ctx: &ElementValuePairContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::ElementValuePair), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#elementValue}.
     * @param ctx the parse tree
     */
    fn enter_elementValue(&mut self, _ctx: &ElementValueContext<'input>) {
        self.stack_mut().push(JavaNode::new(JavaNodeType::ElementValue));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#elementValue}.
     * @param ctx the parse tree
     */
    fn exit_elementValue(&mut self, _ctx: &ElementValueContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::ElementValue), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#elementValueArrayInitializer}.
     * @param ctx the parse tree
     */
    fn enter_elementValueArrayInitializer(
        &mut self,
        _ctx: &ElementValueArrayInitializerContext<'input>,
    ) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::ElementValueArrayInitializer));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#elementValueArrayInitializer}.
     * @param ctx the parse tree
     */
    fn exit_elementValueArrayInitializer(
        &mut self,
        _ctx: &ElementValueArrayInitializerContext<'input>,
    ) {
        self.update_node_attrs(Some(JavaNodeType::ElementValueArrayInitializer), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#annotationTypeDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_annotationTypeDeclaration(&mut self, _ctx: &AnnotationTypeDeclarationContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::AnnotationTypeDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#annotationTypeDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_annotationTypeDeclaration(&mut self, _ctx: &AnnotationTypeDeclarationContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::AnnotationTypeDeclaration), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#annotationTypeBody}.
     * @param ctx the parse tree
     */
    fn enter_annotationTypeBody(&mut self, _ctx: &AnnotationTypeBodyContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::AnnotationTypeBody));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#annotationTypeBody}.
     * @param ctx the parse tree
     */
    fn exit_annotationTypeBody(&mut self, _ctx: &AnnotationTypeBodyContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::AnnotationTypeBody), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#annotationTypeElementDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_annotationTypeElementDeclaration(
        &mut self,
        _ctx: &AnnotationTypeElementDeclarationContext<'input>,
    ) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::AnnotationTypeElementDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#annotationTypeElementDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_annotationTypeElementDeclaration(
        &mut self,
        _ctx: &AnnotationTypeElementDeclarationContext<'input>,
    ) {
        self.update_node_attrs(Some(JavaNodeType::AnnotationTypeElementDeclaration), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#annotationTypeElementRest}.
     * @param ctx the parse tree
     */
    fn enter_annotationTypeElementRest(&mut self, _ctx: &AnnotationTypeElementRestContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::AnnotationTypeElementRest));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#annotationTypeElementRest}.
     * @param ctx the parse tree
     */
    fn exit_annotationTypeElementRest(&mut self, _ctx: &AnnotationTypeElementRestContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::AnnotationTypeElementRest), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#annotationMethodOrConstantRest}.
     * @param ctx the parse tree
     */
    fn enter_annotationMethodOrConstantRest(
        &mut self,
        _ctx: &AnnotationMethodOrConstantRestContext<'input>,
    ) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::AnnotationMethodOrConstantRest));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#annotationMethodOrConstantRest}.
     * @param ctx the parse tree
     */
    fn exit_annotationMethodOrConstantRest(
        &mut self,
        _ctx: &AnnotationMethodOrConstantRestContext<'input>,
    ) {
        self.update_node_attrs(Some(JavaNodeType::AnnotationMethodOrConstantRest), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#annotationMethodRest}.
     * @param ctx the parse tree
     */
    fn enter_annotationMethodRest(&mut self, _ctx: &AnnotationMethodRestContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::AnnotationMethodRest));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#annotationMethodRest}.
     * @param ctx the parse tree
     */
    fn exit_annotationMethodRest(&mut self, _ctx: &AnnotationMethodRestContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::AnnotationMethodRest), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#annotationConstantRest}.
     * @param ctx the parse tree
     */
    fn enter_annotationConstantRest(&mut self, _ctx: &AnnotationConstantRestContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::AnnotationConstantRest));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#annotationConstantRest}.
     * @param ctx the parse tree
     */
    fn exit_annotationConstantRest(&mut self, _ctx: &AnnotationConstantRestContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::AnnotationConstantRest), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#defaultValue}.
     * @param ctx the parse tree
     */
    fn enter_defaultValue(&mut self, _ctx: &DefaultValueContext<'input>) {
        self.stack_mut().push(JavaNode::new(JavaNodeType::DefaultValue));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#defaultValue}.
     * @param ctx the parse tree
     */
    fn exit_defaultValue(&mut self, _ctx: &DefaultValueContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::DefaultValue), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#moduleDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_moduleDeclaration(&mut self, _ctx: &ModuleDeclarationContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::ModuleDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#moduleDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_moduleDeclaration(&mut self, _ctx: &ModuleDeclarationContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::ModuleDeclaration), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#moduleBody}.
     * @param ctx the parse tree
     */
    fn enter_moduleBody(&mut self, _ctx: &ModuleBodyContext<'input>) {
        self.stack_mut().push(JavaNode::new(JavaNodeType::ModuleBody));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#moduleBody}.
     * @param ctx the parse tree
     */
    fn exit_moduleBody(&mut self, _ctx: &ModuleBodyContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::ModuleBody), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#moduleDirective}.
     * @param ctx the parse tree
     */
    fn enter_moduleDirective(&mut self, _ctx: &ModuleDirectiveContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::ModuleDirective));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#moduleDirective}.
     * @param ctx the parse tree
     */
    fn exit_moduleDirective(&mut self, _ctx: &ModuleDirectiveContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::ModuleDirective), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#requiresModifier}.
     * @param ctx the parse tree
     */
    fn enter_requiresModifier(&mut self, _ctx: &RequiresModifierContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::RequiresModifier));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#requiresModifier}.
     * @param ctx the parse tree
     */
    fn exit_requiresModifier(&mut self, _ctx: &RequiresModifierContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::RequiresModifier), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#recordDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_recordDeclaration(&mut self, _ctx: &RecordDeclarationContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::RecordDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#recordDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_recordDeclaration(&mut self, _ctx: &RecordDeclarationContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::RecordDeclaration), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#recordHeader}.
     * @param ctx the parse tree
     */
    fn enter_recordHeader(&mut self, _ctx: &RecordHeaderContext<'input>) {
        self.stack_mut().push(JavaNode::new(JavaNodeType::RecordHeader));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#recordHeader}.
     * @param ctx the parse tree
     */
    fn exit_recordHeader(&mut self, _ctx: &RecordHeaderContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::RecordHeader), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#recordComponentList}.
     * @param ctx the parse tree
     */
    fn enter_recordComponentList(&mut self, _ctx: &RecordComponentListContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::RecordComponentList));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#recordComponentList}.
     * @param ctx the parse tree
     */
    fn exit_recordComponentList(&mut self, _ctx: &RecordComponentListContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::RecordComponentList), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#recordComponent}.
     * @param ctx the parse tree
     */
    fn enter_recordComponent(&mut self, _ctx: &RecordComponentContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::RecordComponent));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#recordComponent}.
     * @param ctx the parse tree
     */
    fn exit_recordComponent(&mut self, _ctx: &RecordComponentContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::RecordComponent), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#recordBody}.
     * @param ctx the parse tree
     */
    fn enter_recordBody(&mut self, _ctx: &RecordBodyContext<'input>) {
        self.stack_mut().push(JavaNode::new(JavaNodeType::RecordBody));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#recordBody}.
     * @param ctx the parse tree
     */
    fn exit_recordBody(&mut self, _ctx: &RecordBodyContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::RecordBody), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#block}.
     * @param ctx the parse tree
     */
    fn enter_block(&mut self, _ctx: &BlockContext<'input>) {
        self.stack_mut().push(JavaNode::new(JavaNodeType::Block));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#block}.
     * @param ctx the parse tree
     */
    fn exit_block(&mut self, _ctx: &BlockContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::Block), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#blockStatement}.
     * @param ctx the parse tree
     */
    fn enter_blockStatement(&mut self, _ctx: &BlockStatementContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::BlockStatement));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#blockStatement}.
     * @param ctx the parse tree
     */
    fn exit_blockStatement(&mut self, _ctx: &BlockStatementContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::BlockStatement), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#localVariableDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_localVariableDeclaration(&mut self, _ctx: &LocalVariableDeclarationContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::LocalVariableDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#localVariableDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_localVariableDeclaration(&mut self, _ctx: &LocalVariableDeclarationContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::LocalVariableDeclaration), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#identifier}.
     * @param ctx the parse tree
     */
    fn enter_identifier(&mut self, _ctx: &IdentifierContext<'input>) {
        self.stack_mut().push(JavaNode::new(JavaNodeType::Identifier));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#identifier}.
     * @param ctx the parse tree
     */
    fn exit_identifier(&mut self, _ctx: &IdentifierContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::Identifier), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#typeIdentifier}.
     * @param ctx the parse tree
     */
    fn enter_typeIdentifier(&mut self, _ctx: &TypeIdentifierContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::TypeIdentifier));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#typeIdentifier}.
     * @param ctx the parse tree
     */
    fn exit_typeIdentifier(&mut self, _ctx: &TypeIdentifierContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::TypeIdentifier), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#localTypeDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_localTypeDeclaration(&mut self, _ctx: &LocalTypeDeclarationContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::LocalTypeDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#localTypeDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_localTypeDeclaration(&mut self, _ctx: &LocalTypeDeclarationContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::LocalTypeDeclaration), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#statement}.
     * @param ctx the parse tree
     */
    fn enter_statement(&mut self, _ctx: &StatementContext<'input>) {
        self.stack_mut().push(JavaNode::new(JavaNodeType::Statement));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#statement}.
     * @param ctx the parse tree
     */
    fn exit_statement(&mut self, _ctx: &StatementContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::Statement), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#catchClause}.
     * @param ctx the parse tree
     */
    fn enter_catchClause(&mut self, _ctx: &CatchClauseContext<'input>) {
        self.stack_mut().push(JavaNode::new(JavaNodeType::CatchClause));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#catchClause}.
     * @param ctx the parse tree
     */
    fn exit_catchClause(&mut self, _ctx: &CatchClauseContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::CatchClause), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#catchType}.
     * @param ctx the parse tree
     */
    fn enter_catchType(&mut self, _ctx: &CatchTypeContext<'input>) {
        self.stack_mut().push(JavaNode::new(JavaNodeType::CatchType));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#catchType}.
     * @param ctx the parse tree
     */
    fn exit_catchType(&mut self, _ctx: &CatchTypeContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::CatchType), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#finallyBlock}.
     * @param ctx the parse tree
     */
    fn enter_finallyBlock(&mut self, _ctx: &FinallyBlockContext<'input>) {
        self.stack_mut().push(JavaNode::new(JavaNodeType::FinallyBlock));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#finallyBlock}.
     * @param ctx the parse tree
     */
    fn exit_finallyBlock(&mut self, _ctx: &FinallyBlockContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::FinallyBlock), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#resourceSpecification}.
     * @param ctx the parse tree
     */
    fn enter_resourceSpecification(&mut self, _ctx: &ResourceSpecificationContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::ResourceSpecification));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#resourceSpecification}.
     * @param ctx the parse tree
     */
    fn exit_resourceSpecification(&mut self, _ctx: &ResourceSpecificationContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::ResourceSpecification), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#resources}.
     * @param ctx the parse tree
     */
    fn enter_resources(&mut self, _ctx: &ResourcesContext<'input>) {
        self.stack_mut().push(JavaNode::new(JavaNodeType::Resources));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#resources}.
     * @param ctx the parse tree
     */
    fn exit_resources(&mut self, _ctx: &ResourcesContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::Resources), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#resource}.
     * @param ctx the parse tree
     */
    fn enter_resource(&mut self, _ctx: &ResourceContext<'input>) {
        self.stack_mut().push(JavaNode::new(JavaNodeType::Resource));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#resource}.
     * @param ctx the parse tree
     */
    fn exit_resource(&mut self, _ctx: &ResourceContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::Resource), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#switchBlockStatementGroup}.
     * @param ctx the parse tree
     */
    fn enter_switchBlockStatementGroup(&mut self, _ctx: &SwitchBlockStatementGroupContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::SwitchBlockStatementGroup));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#switchBlockStatementGroup}.
     * @param ctx the parse tree
     */
    fn exit_switchBlockStatementGroup(&mut self, _ctx: &SwitchBlockStatementGroupContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::SwitchBlockStatementGroup), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#switchLabel}.
     * @param ctx the parse tree
     */
    fn enter_switchLabel(&mut self, _ctx: &SwitchLabelContext<'input>) {
        self.stack_mut().push(JavaNode::new(JavaNodeType::SwitchLabel));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#switchLabel}.
     * @param ctx the parse tree
     */
    fn exit_switchLabel(&mut self, _ctx: &SwitchLabelContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::SwitchLabel), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#forControl}.
     * @param ctx the parse tree
     */
    fn enter_forControl(&mut self, _ctx: &ForControlContext<'input>) {
        self.stack_mut().push(JavaNode::new(JavaNodeType::ForControl));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#forControl}.
     * @param ctx the parse tree
     */
    fn exit_forControl(&mut self, _ctx: &ForControlContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::ForControl), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#forInit}.
     * @param ctx the parse tree
     */
    fn enter_forInit(&mut self, _ctx: &ForInitContext<'input>) {
        self.stack_mut().push(JavaNode::new(JavaNodeType::ForInit));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#forInit}.
     * @param ctx the parse tree
     */
    fn exit_forInit(&mut self, _ctx: &ForInitContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::ForInit), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#enhancedForControl}.
     * @param ctx the parse tree
     */
    fn enter_enhancedForControl(&mut self, _ctx: &EnhancedForControlContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::EnhancedForControl));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#enhancedForControl}.
     * @param ctx the parse tree
     */
    fn exit_enhancedForControl(&mut self, _ctx: &EnhancedForControlContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::EnhancedForControl), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#parExpression}.
     * @param ctx the parse tree
     */
    fn enter_parExpression(&mut self, _ctx: &ParExpressionContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::ParExpression));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#parExpression}.
     * @param ctx the parse tree
     */
    fn exit_parExpression(&mut self, _ctx: &ParExpressionContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::ParExpression), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#expressionList}.
     * @param ctx the parse tree
     */
    fn enter_expressionList(&mut self, _ctx: &ExpressionListContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::ExpressionList));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#expressionList}.
     * @param ctx the parse tree
     */
    fn exit_expressionList(&mut self, _ctx: &ExpressionListContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::ExpressionList), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#methodCall}.
     * @param ctx the parse tree
     */
    fn enter_methodCall(&mut self, _ctx: &MethodCallContext<'input>) {
        self.stack_mut().push(JavaNode::new(JavaNodeType::MethodCall));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#methodCall}.
     * @param ctx the parse tree
     */
    fn exit_methodCall(&mut self, _ctx: &MethodCallContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::MethodCall), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#expression}.
     * @param ctx the parse tree
     */
    fn enter_expression(&mut self, _ctx: &ExpressionContext<'input>) {
        self.stack_mut().push(JavaNode::new(JavaNodeType::Expression));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#expression}.
     * @param ctx the parse tree
     */
    fn exit_expression(&mut self, _ctx: &ExpressionContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::Expression), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#pattern}.
     * @param ctx the parse tree
     */
    fn enter_pattern(&mut self, _ctx: &PatternContext<'input>) {
        self.stack_mut().push(JavaNode::new(JavaNodeType::Pattern));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#pattern}.
     * @param ctx the parse tree
     */
    fn exit_pattern(&mut self, _ctx: &PatternContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::Pattern), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#lambdaExpression}.
     * @param ctx the parse tree
     */
    fn enter_lambdaExpression(&mut self, _ctx: &LambdaExpressionContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::LambdaExpression));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#lambdaExpression}.
     * @param ctx the parse tree
     */
    fn exit_lambdaExpression(&mut self, _ctx: &LambdaExpressionContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::LambdaExpression), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#lambdaParameters}.
     * @param ctx the parse tree
     */
    fn enter_lambdaParameters(&mut self, _ctx: &LambdaParametersContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::LambdaParameters));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#lambdaParameters}.
     * @param ctx the parse tree
     */
    fn exit_lambdaParameters(&mut self, _ctx: &LambdaParametersContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::LambdaParameters), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#lambdaBody}.
     * @param ctx the parse tree
     */
    fn enter_lambdaBody(&mut self, _ctx: &LambdaBodyContext<'input>) {
        self.stack_mut().push(JavaNode::new(JavaNodeType::LambdaBody));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#lambdaBody}.
     * @param ctx the parse tree
     */
    fn exit_lambdaBody(&mut self, _ctx: &LambdaBodyContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::LambdaBody), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#primary}.
     * @param ctx the parse tree
     */
    fn enter_primary(&mut self, _ctx: &PrimaryContext<'input>) {
        self.stack_mut().push(JavaNode::new(JavaNodeType::Primary));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#primary}.
     * @param ctx the parse tree
     */
    fn exit_primary(&mut self, _ctx: &PrimaryContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::Primary), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#switchExpression}.
     * @param ctx the parse tree
     */
    fn enter_switchExpression(&mut self, _ctx: &SwitchExpressionContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::SwitchExpression));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#switchExpression}.
     * @param ctx the parse tree
     */
    fn exit_switchExpression(&mut self, _ctx: &SwitchExpressionContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::SwitchExpression), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#switchLabeledRule}.
     * @param ctx the parse tree
     */
    fn enter_switchLabeledRule(&mut self, _ctx: &SwitchLabeledRuleContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::SwitchLabeledRule));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#switchLabeledRule}.
     * @param ctx the parse tree
     */
    fn exit_switchLabeledRule(&mut self, _ctx: &SwitchLabeledRuleContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::SwitchLabeledRule), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#guardedPattern}.
     * @param ctx the parse tree
     */
    fn enter_guardedPattern(&mut self, _ctx: &GuardedPatternContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::GuardedPattern));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#guardedPattern}.
     * @param ctx the parse tree
     */
    fn exit_guardedPattern(&mut self, _ctx: &GuardedPatternContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::GuardedPattern), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#switchRuleOutcome}.
     * @param ctx the parse tree
     */
    fn enter_switchRuleOutcome(&mut self, _ctx: &SwitchRuleOutcomeContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::SwitchRuleOutcome));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#switchRuleOutcome}.
     * @param ctx the parse tree
     */
    fn exit_switchRuleOutcome(&mut self, _ctx: &SwitchRuleOutcomeContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::SwitchRuleOutcome), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#classType}.
     * @param ctx the parse tree
     */
    fn enter_classType(&mut self, _ctx: &ClassTypeContext<'input>) {
        self.stack_mut().push(JavaNode::new(JavaNodeType::ClassType));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#classType}.
     * @param ctx the parse tree
     */
    fn exit_classType(&mut self, _ctx: &ClassTypeContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::ClassType), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#creator}.
     * @param ctx the parse tree
     */
    fn enter_creator(&mut self, _ctx: &CreatorContext<'input>) {
        self.stack_mut().push(JavaNode::new(JavaNodeType::Creator));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#creator}.
     * @param ctx the parse tree
     */
    fn exit_creator(&mut self, _ctx: &CreatorContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::Creator), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#createdName}.
     * @param ctx the parse tree
     */
    fn enter_createdName(&mut self, _ctx: &CreatedNameContext<'input>) {
        self.stack_mut().push(JavaNode::new(JavaNodeType::CreatedName));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#createdName}.
     * @param ctx the parse tree
     */
    fn exit_createdName(&mut self, _ctx: &CreatedNameContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::CreatedName), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#innerCreator}.
     * @param ctx the parse tree
     */
    fn enter_innerCreator(&mut self, _ctx: &InnerCreatorContext<'input>) {
        self.stack_mut().push(JavaNode::new(JavaNodeType::InnerCreator));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#innerCreator}.
     * @param ctx the parse tree
     */
    fn exit_innerCreator(&mut self, _ctx: &InnerCreatorContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::InnerCreator), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#arrayCreatorRest}.
     * @param ctx the parse tree
     */
    fn enter_arrayCreatorRest(&mut self, _ctx: &ArrayCreatorRestContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::ArrayCreatorRest));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#arrayCreatorRest}.
     * @param ctx the parse tree
     */
    fn exit_arrayCreatorRest(&mut self, _ctx: &ArrayCreatorRestContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::ArrayCreatorRest), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#classCreatorRest}.
     * @param ctx the parse tree
     */
    fn enter_classCreatorRest(&mut self, _ctx: &ClassCreatorRestContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::ClassCreatorRest));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#classCreatorRest}.
     * @param ctx the parse tree
     */
    fn exit_classCreatorRest(&mut self, _ctx: &ClassCreatorRestContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::ClassCreatorRest), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#explicitGenericInvocation}.
     * @param ctx the parse tree
     */
    fn enter_explicitGenericInvocation(&mut self, _ctx: &ExplicitGenericInvocationContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::ExplicitGenericInvocation));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#explicitGenericInvocation}.
     * @param ctx the parse tree
     */
    fn exit_explicitGenericInvocation(&mut self, _ctx: &ExplicitGenericInvocationContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::ExplicitGenericInvocation), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#typeArgumentsOrDiamond}.
     * @param ctx the parse tree
     */
    fn enter_typeArgumentsOrDiamond(&mut self, _ctx: &TypeArgumentsOrDiamondContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::TypeArgumentsOrDiamond));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#typeArgumentsOrDiamond}.
     * @param ctx the parse tree
     */
    fn exit_typeArgumentsOrDiamond(&mut self, _ctx: &TypeArgumentsOrDiamondContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::TypeArgumentsOrDiamond), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#nonWildcardTypeArgumentsOrDiamond}.
     * @param ctx the parse tree
     */
    fn enter_nonWildcardTypeArgumentsOrDiamond(
        &mut self,
        _ctx: &NonWildcardTypeArgumentsOrDiamondContext<'input>,
    ) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::NonWildcardTypeArgumentsOrDiamond));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#nonWildcardTypeArgumentsOrDiamond}.
     * @param ctx the parse tree
     */
    fn exit_nonWildcardTypeArgumentsOrDiamond(
        &mut self,
        _ctx: &NonWildcardTypeArgumentsOrDiamondContext<'input>,
    ) {
        self.update_node_attrs(Some(JavaNodeType::NonWildcardTypeArgumentsOrDiamond), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#nonWildcardTypeArguments}.
     * @param ctx the parse tree
     */
    fn enter_nonWildcardTypeArguments(&mut self, _ctx: &NonWildcardTypeArgumentsContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::NonWildcardTypeArguments));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#nonWildcardTypeArguments}.
     * @param ctx the parse tree
     */
    fn exit_nonWildcardTypeArguments(&mut self, _ctx: &NonWildcardTypeArgumentsContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::NonWildcardTypeArguments), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#typeList}.
     * @param ctx the parse tree
     */
    fn enter_typeList(&mut self, _ctx: &TypeListContext<'input>) {
        self.stack_mut().push(JavaNode::new(JavaNodeType::TypeList));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#typeList}.
     * @param ctx the parse tree
     */
    fn exit_typeList(&mut self, _ctx: &TypeListContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::TypeList), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#typeType}.
     * @param ctx the parse tree
     */
    fn enter_typeType(&mut self, _ctx: &TypeTypeContext<'input>) {
        self.stack_mut().push(JavaNode::new(JavaNodeType::TypeType));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#typeType}.
     * @param ctx the parse tree
     */
    fn exit_typeType(&mut self, _ctx: &TypeTypeContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::TypeType), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#primitiveType}.
     * @param ctx the parse tree
     */
    fn enter_primitiveType(&mut self, _ctx: &PrimitiveTypeContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::PrimitiveType));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#primitiveType}.
     * @param ctx the parse tree
     */
    fn exit_primitiveType(&mut self, _ctx: &PrimitiveTypeContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::PrimitiveType), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#typeArguments}.
     * @param ctx the parse tree
     */
    fn enter_typeArguments(&mut self, _ctx: &TypeArgumentsContext<'input>) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::TypeArguments));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#typeArguments}.
     * @param ctx the parse tree
     */
    fn exit_typeArguments(&mut self, _ctx: &TypeArgumentsContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::TypeArguments), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#superSuffix}.
     * @param ctx the parse tree
     */
    fn enter_superSuffix(&mut self, _ctx: &SuperSuffixContext<'input>) {
        self.stack_mut().push(JavaNode::new(JavaNodeType::SuperSuffix));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#superSuffix}.
     * @param ctx the parse tree
     */
    fn exit_superSuffix(&mut self, _ctx: &SuperSuffixContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::SuperSuffix), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#explicitGenericInvocationSuffix}.
     * @param ctx the parse tree
     */
    fn enter_explicitGenericInvocationSuffix(
        &mut self,
        _ctx: &ExplicitGenericInvocationSuffixContext<'input>,
    ) {
        self.stack_mut()
            .push(JavaNode::new(JavaNodeType::ExplicitGenericInvocationSuffix));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#explicitGenericInvocationSuffix}.
     * @param ctx the parse tree
     */
    fn exit_explicitGenericInvocationSuffix(
        &mut self,
        _ctx: &ExplicitGenericInvocationSuffixContext<'input>,
    ) {
        self.update_node_attrs(Some(JavaNodeType::ExplicitGenericInvocationSuffix), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#arguments}.
     * @param ctx the parse tree
     */
    fn enter_arguments(&mut self, _ctx: &ArgumentsContext<'input>) {
        self.stack_mut().push(JavaNode::new(JavaNodeType::Arguments));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#arguments}.
     * @param ctx the parse tree
     */
    fn exit_arguments(&mut self, _ctx: &ArgumentsContext<'input>) {
        self.update_node_attrs(Some(JavaNodeType::Arguments), |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
}
