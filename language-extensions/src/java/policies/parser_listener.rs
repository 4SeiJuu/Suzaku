use antlr_rust::{
    parser::ParserNodeType,
    tree::{
        ErrorNode, 
        ParseTree, 
        ParseTreeListener, 
        TerminalNode
    },
};

use suzaku_extension_sdk::{
    stack::Stack,
    meta::{
        IMeta, 
        Metadata
    },
    meta_type::MetaType,
    reorganzier::LanguageMetaReorganizePolicy, 
    parser::LanguageParserListener,
};

use super::{
    super::generated::{
        javaparserlistener::JavaParserListener,
        javaparser::*
    },
};

pub struct ParserListener {
    stack: Stack<Metadata>,
    reorganizer: Option<Box<dyn LanguageMetaReorganizePolicy>>
}

impl LanguageParserListener for ParserListener {
    fn new(root_node: Metadata, reorganizer: Option<Box<dyn LanguageMetaReorganizePolicy>>) -> Self {
        let mut st = Stack::new();
        st.push(root_node);
        ParserListener {
            stack: st,
            reorganizer: reorganizer
        }
    }

    fn results(&mut self) -> Option<Metadata> {
        self.stack_mut().pop()
    }


    fn stack(&self) -> &Stack<Metadata> {
        &&self.stack
    }

    fn stack_mut(&mut self) -> &mut Stack<Metadata> {
        &mut self.stack
    }

    fn reorganizer(&mut self) -> Option<&mut dyn LanguageMetaReorganizePolicy> {
        match &mut self.reorganizer {
            Some(ref mut reorg) => Some(reorg.as_mut()),
            None => None
        }
    }
}

impl<'input, 'a, Node: ParserNodeType<'input>> ParseTreeListener<'input, Node> for ParserListener {
    /// Called when parser creates terminal node
    fn visit_terminal(&mut self, _node: &TerminalNode<'input, Node>) {
        match _node.get_text().as_str() {
            "=" | ">" | "<" | "!" | "~" | "?" | ":" | "==" | "<=" | ">=" | "!=" | "&&" | "||"
            | "++" | "--" | "+" | "-" | "*" | "/" | "&" | "|" | "^" | "%" | "+=" | "-=" | "*="
            | "/=" | "&=" | "|=" | "^=" | "%=" | "<<=" | ">>=" | ">>>=" | "->" | "::" => {
                let mut op_node = Metadata::new(MetaType::Operator);
                op_node.set_attr(_node.get_text().as_str());
                self.stack_mut()
                    .top_mut()
                    .unwrap()
                    .get_members_mut()
                    .push_back(op_node);
            },
            "," => {
                let mut sep_node = Metadata::new(MetaType::Separator);
                sep_node.set_attr(_node.get_text().as_str());
                self.stack_mut()
                    .top_mut()
                    .unwrap()
                    .get_members_mut()
                    .push_back(sep_node);
            }
            "static" => match self.stack_mut().top_mut().unwrap().get_node_type() {
                MetaType::ImportDeclaration | MetaType::ClassBodyDeclaration => {
                    if let Some(top_node) = self.stack_mut().top_mut() {
                        let mut modifier_node = Metadata::new(MetaType::Modifier);
                        modifier_node.set_attr(_node.get_text().as_str());
                        top_node.get_members_mut().push_back(modifier_node);
                    }
                }
                _ => (),
            },
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
            .push(Metadata::new(MetaType::PackageDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#packageDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_packageDeclaration(&mut self, _ctx: &PackageDeclarationContext<'input>) {
        self.update_node_attrs(MetaType::PackageDeclaration, |node| {
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
            .push(Metadata::new(MetaType::ImportDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#importDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_importDeclaration(&mut self, _ctx: &ImportDeclarationContext<'input>) {
        self.update_node_attrs(MetaType::ImportDeclaration, |node| {
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
            .push(Metadata::new(MetaType::TypeDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#typeDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_typeDeclaration(&mut self, _ctx: &TypeDeclarationContext<'input>) {
        self.update_node_attrs(MetaType::TypeDeclaration, |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#modifier}.
     * @param ctx the parse tree
     */
    fn enter_modifier(&mut self, _ctx: &ModifierContext<'input>) {
        self.stack_mut().push(Metadata::new(MetaType::Modifier));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#modifier}.
     * @param ctx the parse tree
     */
    fn exit_modifier(&mut self, _ctx: &ModifierContext<'input>) {
        self.update_node_attrs(MetaType::Modifier, |node| {
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
            .push(Metadata::new(MetaType::VariableModifier));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#variableModifier}.
     * @param ctx the parse tree
     */
    fn exit_variableModifier(&mut self, _ctx: &VariableModifierContext<'input>) {
        self.update_node_attrs(MetaType::VariableModifier, |node| {
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
            .push(Metadata::new(MetaType::ClassDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#classDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_classDeclaration(&mut self, _ctx: &ClassDeclarationContext<'input>) {
        self.update_node_attrs(MetaType::ClassDeclaration, |node| {
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
            .push(Metadata::new(MetaType::TypeParameters));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#typeParameters}.
     * @param ctx the parse tree
     */
    fn exit_typeParameters(&mut self, _ctx: &TypeParametersContext<'input>) {
        self.update_node_attrs(MetaType::TypeParameters, |node| {
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
            .push(Metadata::new(MetaType::TypeParameter));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#typeParameter}.
     * @param ctx the parse tree
     */
    fn exit_typeParameter(&mut self, _ctx: &TypeParameterContext<'input>) {
        self.update_node_attrs(MetaType::TypeParameter, |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#typeBound}.
     * @param ctx the parse tree
     */
    fn enter_typeBound(&mut self, _ctx: &TypeBoundContext<'input>) {
        self.stack_mut().push(Metadata::new(MetaType::TypeBound));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#typeBound}.
     * @param ctx the parse tree
     */
    fn exit_typeBound(&mut self, _ctx: &TypeBoundContext<'input>) {
        self.update_node_attrs(MetaType::TypeBound, |node| {
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
            .push(Metadata::new(MetaType::EnumDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#enumDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_enumDeclaration(&mut self, _ctx: &EnumDeclarationContext<'input>) {
        self.update_node_attrs(MetaType::EnumDeclaration, |node| {
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
            .push(Metadata::new(MetaType::EnumConstants));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#enumConstants}.
     * @param ctx the parse tree
     */
    fn exit_enumConstants(&mut self, _ctx: &EnumConstantsContext<'input>) {
        self.update_node_attrs(MetaType::EnumConstants, |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#enumConstant}.
     * @param ctx the parse tree
     */
    fn enter_enumConstant(&mut self, _ctx: &EnumConstantContext<'input>) {
        self.stack_mut().push(Metadata::new(MetaType::EnumConstant));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#enumConstant}.
     * @param ctx the parse tree
     */
    fn exit_enumConstant(&mut self, _ctx: &EnumConstantContext<'input>) {
        self.update_node_attrs(MetaType::EnumConstant, |node| {
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
            .push(Metadata::new(MetaType::EnumBodyDeclarations));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#enumBodyDeclarations}.
     * @param ctx the parse tree
     */
    fn exit_enumBodyDeclarations(&mut self, _ctx: &EnumBodyDeclarationsContext<'input>) {
        self.update_node_attrs(MetaType::EnumBodyDeclarations, |node| {
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
            .push(Metadata::new(MetaType::InterfaceDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#interfaceDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_interfaceDeclaration(&mut self, _ctx: &InterfaceDeclarationContext<'input>) {
        self.update_node_attrs(MetaType::InterfaceDeclaration, |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#classBody}.
     * @param ctx the parse tree
     */
    fn enter_classBody(&mut self, _ctx: &ClassBodyContext<'input>) {
        self.stack_mut().push(Metadata::new(MetaType::ClassBody));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#classBody}.
     * @param ctx the parse tree
     */
    fn exit_classBody(&mut self, _ctx: &ClassBodyContext<'input>) {
        self.update_node_attrs(MetaType::ClassBody, |node| {
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
            .push(Metadata::new(MetaType::InterfaceBody));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#interfaceBody}.
     * @param ctx the parse tree
     */
    fn exit_interfaceBody(&mut self, _ctx: &InterfaceBodyContext<'input>) {
        self.update_node_attrs(MetaType::InterfaceBody, |node| {
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
            .push(Metadata::new(MetaType::ClassBodyDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#classBodyDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_classBodyDeclaration(&mut self, _ctx: &ClassBodyDeclarationContext<'input>) {
        self.update_node_attrs(MetaType::ClassBodyDeclaration, |node| {
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
            .push(Metadata::new(MetaType::MemberDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#memberDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_memberDeclaration(&mut self, _ctx: &MemberDeclarationContext<'input>) {
        self.update_node_attrs(MetaType::MemberDeclaration, |node| {
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
            .push(Metadata::new(MetaType::MethodDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#methodDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_methodDeclaration(&mut self, _ctx: &MethodDeclarationContext<'input>) {
        self.update_node_attrs(MetaType::MethodDeclaration, |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#methodBody}.
     * @param ctx the parse tree
     */
    fn enter_methodBody(&mut self, _ctx: &MethodBodyContext<'input>) {
        self.stack_mut().push(Metadata::new(MetaType::MethodBody));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#methodBody}.
     * @param ctx the parse tree
     */
    fn exit_methodBody(&mut self, _ctx: &MethodBodyContext<'input>) {
        self.update_node_attrs(MetaType::MethodBody, |node| {
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
            .push(Metadata::new(MetaType::TypeTypeOrVoid));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#typeTypeOrVoid}.
     * @param ctx the parse tree
     */
    fn exit_typeTypeOrVoid(&mut self, _ctx: &TypeTypeOrVoidContext<'input>) {
        self.update_node_attrs(MetaType::TypeTypeOrVoid, |node| {
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
            .push(Metadata::new(MetaType::GenericMethodDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#genericMethodDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_genericMethodDeclaration(&mut self, _ctx: &GenericMethodDeclarationContext<'input>) {
        self.update_node_attrs(MetaType::GenericMethodDeclaration, |node| {
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
            .push(Metadata::new(MetaType::GenericConstructorDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#genericConstructorDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_genericConstructorDeclaration(
        &mut self,
        _ctx: &GenericConstructorDeclarationContext<'input>,
    ) {
        self.update_node_attrs(MetaType::GenericConstructorDeclaration, |node| {
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
            .push(Metadata::new(MetaType::ConstructorDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#constructorDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_constructorDeclaration(&mut self, _ctx: &ConstructorDeclarationContext<'input>) {
        self.update_node_attrs(MetaType::ConstructorDeclaration, |node| {
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
            .push(Metadata::new(MetaType::CompactConstructorDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#compactConstructorDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_compactConstructorDeclaration(
        &mut self,
        _ctx: &CompactConstructorDeclarationContext<'input>,
    ) {
        self.update_node_attrs(MetaType::CompactConstructorDeclaration, |node| {
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
            .push(Metadata::new(MetaType::FieldDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#fieldDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_fieldDeclaration(&mut self, _ctx: &FieldDeclarationContext<'input>) {
        self.update_node_attrs(MetaType::FieldDeclaration, |node| {
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
            .push(Metadata::new(MetaType::InterfaceBodyDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#interfaceBodyDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_interfaceBodyDeclaration(&mut self, _ctx: &InterfaceBodyDeclarationContext<'input>) {
        self.update_node_attrs(MetaType::InterfaceBodyDeclaration, |node| {
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
            .push(Metadata::new(MetaType::InterfaceMemberDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#interfaceMemberDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_interfaceMemberDeclaration(
        &mut self,
        _ctx: &InterfaceMemberDeclarationContext<'input>,
    ) {
        self.update_node_attrs(MetaType::InterfaceMemberDeclaration, |node| {
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
            .push(Metadata::new(MetaType::ConstDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#constDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_constDeclaration(&mut self, _ctx: &ConstDeclarationContext<'input>) {
        self.update_node_attrs(MetaType::ConstDeclaration, |node| {
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
            .push(Metadata::new(MetaType::ConstantDeclarator));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#constantDeclarator}.
     * @param ctx the parse tree
     */
    fn exit_constantDeclarator(&mut self, _ctx: &ConstantDeclaratorContext<'input>) {
        self.update_node_attrs(MetaType::ConstantDeclarator, |node| {
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
            .push(Metadata::new(MetaType::InterfaceMethodDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#interfaceMethodDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_interfaceMethodDeclaration(
        &mut self,
        _ctx: &InterfaceMethodDeclarationContext<'input>,
    ) {
        self.update_node_attrs(MetaType::InterfaceMethodDeclaration, |node| {
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
            .push(Metadata::new(MetaType::InterfaceMethodModifier));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#interfaceMethodModifier}.
     * @param ctx the parse tree
     */
    fn exit_interfaceMethodModifier(&mut self, _ctx: &InterfaceMethodModifierContext<'input>) {
        self.update_node_attrs(MetaType::InterfaceMethodModifier, |node| {
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
            .push(Metadata::new(MetaType::GenericInterfaceMethodDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#genericInterfaceMethodDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_genericInterfaceMethodDeclaration(
        &mut self,
        _ctx: &GenericInterfaceMethodDeclarationContext<'input>,
    ) {
        self.update_node_attrs(MetaType::GenericInterfaceMethodDeclaration, |node| {
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
            .push(Metadata::new(MetaType::InterfaceCommonBodyDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#interfaceCommonBodyDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_interfaceCommonBodyDeclaration(
        &mut self,
        _ctx: &InterfaceCommonBodyDeclarationContext<'input>,
    ) {
        self.update_node_attrs(MetaType::InterfaceCommonBodyDeclaration, |node| {
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
            .push(Metadata::new(MetaType::VariableDeclarators));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#variableDeclarators}.
     * @param ctx the parse tree
     */
    fn exit_variableDeclarators(&mut self, _ctx: &VariableDeclaratorsContext<'input>) {
        self.update_node_attrs(MetaType::VariableDeclarators, |node| {
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
            .push(Metadata::new(MetaType::VariableDeclarator));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#variableDeclarator}.
     * @param ctx the parse tree
     */
    fn exit_variableDeclarator(&mut self, _ctx: &VariableDeclaratorContext<'input>) {
        self.update_node_attrs(MetaType::VariableDeclarator, |node| {
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
            .push(Metadata::new(MetaType::VariableDeclaratorId));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#variableDeclaratorId}.
     * @param ctx the parse tree
     */
    fn exit_variableDeclaratorId(&mut self, _ctx: &VariableDeclaratorIdContext<'input>) {
        self.update_node_attrs(MetaType::VariableDeclaratorId, |node| {
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
            .push(Metadata::new(MetaType::VariableInitializer));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#variableInitializer}.
     * @param ctx the parse tree
     */
    fn exit_variableInitializer(&mut self, _ctx: &VariableInitializerContext<'input>) {
        self.update_node_attrs(MetaType::VariableInitializer, |node| {
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
            .push(Metadata::new(MetaType::ArrayInitializer));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#arrayInitializer}.
     * @param ctx the parse tree
     */
    fn exit_arrayInitializer(&mut self, _ctx: &ArrayInitializerContext<'input>) {
        self.update_node_attrs(MetaType::ArrayInitializer, |node| {
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
            .push(Metadata::new(MetaType::ClassOrInterfaceType));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#classOrInterfaceType}.
     * @param ctx the parse tree
     */
    fn exit_classOrInterfaceType(&mut self, _ctx: &ClassOrInterfaceTypeContext<'input>) {
        self.update_node_attrs(MetaType::ClassOrInterfaceType, |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#typeArgument}.
     * @param ctx the parse tree
     */
    fn enter_typeArgument(&mut self, _ctx: &TypeArgumentContext<'input>) {
        self.stack_mut().push(Metadata::new(MetaType::TypeArgument));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#typeArgument}.
     * @param ctx the parse tree
     */
    fn exit_typeArgument(&mut self, _ctx: &TypeArgumentContext<'input>) {
        self.update_node_attrs(MetaType::TypeArgument, |node| {
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
            .push(Metadata::new(MetaType::QualifiedNameList));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#qualifiedNameList}.
     * @param ctx the parse tree
     */
    fn exit_qualifiedNameList(&mut self, _ctx: &QualifiedNameListContext<'input>) {
        self.update_node_attrs(MetaType::QualifiedNameList, |node| {
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
            .push(Metadata::new(MetaType::FormalParameters));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#formalParameters}.
     * @param ctx the parse tree
     */
    fn exit_formalParameters(&mut self, _ctx: &FormalParametersContext<'input>) {
        self.update_node_attrs(MetaType::FormalParameters, |node| {
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
            .push(Metadata::new(MetaType::ReceiverParameter));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#receiverParameter}.
     * @param ctx the parse tree
     */
    fn exit_receiverParameter(&mut self, _ctx: &ReceiverParameterContext<'input>) {
        self.update_node_attrs(MetaType::ReceiverParameter, |node| {
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
            .push(Metadata::new(MetaType::FormalParameterList));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#formalParameterList}.
     * @param ctx the parse tree
     */
    fn exit_formalParameterList(&mut self, _ctx: &FormalParameterListContext<'input>) {
        self.update_node_attrs(MetaType::FormalParameterList, |node| {
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
            .push(Metadata::new(MetaType::FormalParameter));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#formalParameter}.
     * @param ctx the parse tree
     */
    fn exit_formalParameter(&mut self, _ctx: &FormalParameterContext<'input>) {
        self.update_node_attrs(MetaType::FormalParameter, |node| {
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
            .push(Metadata::new(MetaType::LastFormalParameter));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#lastFormalParameter}.
     * @param ctx the parse tree
     */
    fn exit_lastFormalParameter(&mut self, _ctx: &LastFormalParameterContext<'input>) {
        self.update_node_attrs(MetaType::LastFormalParameter, |node| {
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
            .push(Metadata::new(MetaType::LambdaLVTIList));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#lambdaLVTIList}.
     * @param ctx the parse tree
     */
    fn exit_lambdaLVTIList(&mut self, _ctx: &LambdaLVTIListContext<'input>) {
        self.update_node_attrs(MetaType::LambdaLVTIList, |node| {
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
            .push(Metadata::new(MetaType::LambdaLVTIParameter));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#lambdaLVTIParameter}.
     * @param ctx the parse tree
     */
    fn exit_lambdaLVTIParameter(&mut self, _ctx: &LambdaLVTIParameterContext<'input>) {
        self.update_node_attrs(MetaType::LambdaLVTIParameter, |node| {
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
            .push(Metadata::new(MetaType::QualifiedName));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#qualifiedName}.
     * @param ctx the parse tree
     */
    fn exit_qualifiedName(&mut self, _ctx: &QualifiedNameContext<'input>) {
        self.update_node_attrs(MetaType::QualifiedName, |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#literal}.
     * @param ctx the parse tree
     */
    fn enter_literal(&mut self, _ctx: &LiteralContext<'input>) {
        self.stack_mut().push(Metadata::new(MetaType::Literal));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#literal}.
     * @param ctx the parse tree
     */
    fn exit_literal(&mut self, _ctx: &LiteralContext<'input>) {
        self.update_node_attrs(MetaType::Literal, |node| {
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
            .push(Metadata::new(MetaType::IntegerLiteral));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#integerLiteral}.
     * @param ctx the parse tree
     */
    fn exit_integerLiteral(&mut self, _ctx: &IntegerLiteralContext<'input>) {
        self.update_node_attrs(MetaType::IntegerLiteral, |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#floatLiteral}.
     * @param ctx the parse tree
     */
    fn enter_floatLiteral(&mut self, _ctx: &FloatLiteralContext<'input>) {
        self.stack_mut().push(Metadata::new(MetaType::FloatLiteral));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#floatLiteral}.
     * @param ctx the parse tree
     */
    fn exit_floatLiteral(&mut self, _ctx: &FloatLiteralContext<'input>) {
        self.update_node_attrs(MetaType::FloatLiteral, |node| {
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
            .push(Metadata::new(MetaType::AltAnnotationQualifiedName));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#altAnnotationQualifiedName}.
     * @param ctx the parse tree
     */
    fn exit_altAnnotationQualifiedName(
        &mut self,
        _ctx: &AltAnnotationQualifiedNameContext<'input>,
    ) {
        self.update_node_attrs(MetaType::AltAnnotationQualifiedName, |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#annotation}.
     * @param ctx the parse tree
     */
    fn enter_annotation(&mut self, _ctx: &AnnotationContext<'input>) {
        self.stack_mut().push(Metadata::new(MetaType::Annotation));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#annotation}.
     * @param ctx the parse tree
     */
    fn exit_annotation(&mut self, _ctx: &AnnotationContext<'input>) {
        self.update_node_attrs(MetaType::Annotation, |node| {
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
            .push(Metadata::new(MetaType::ElementValuePairs));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#elementValuePairs}.
     * @param ctx the parse tree
     */
    fn exit_elementValuePairs(&mut self, _ctx: &ElementValuePairsContext<'input>) {
        self.update_node_attrs(MetaType::ElementValuePairs, |node| {
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
            .push(Metadata::new(MetaType::ElementValuePair));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#elementValuePair}.
     * @param ctx the parse tree
     */
    fn exit_elementValuePair(&mut self, _ctx: &ElementValuePairContext<'input>) {
        self.update_node_attrs(MetaType::ElementValuePair, |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#elementValue}.
     * @param ctx the parse tree
     */
    fn enter_elementValue(&mut self, _ctx: &ElementValueContext<'input>) {
        self.stack_mut().push(Metadata::new(MetaType::ElementValue));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#elementValue}.
     * @param ctx the parse tree
     */
    fn exit_elementValue(&mut self, _ctx: &ElementValueContext<'input>) {
        self.update_node_attrs(MetaType::ElementValue, |node| {
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
            .push(Metadata::new(MetaType::ElementValueArrayInitializer));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#elementValueArrayInitializer}.
     * @param ctx the parse tree
     */
    fn exit_elementValueArrayInitializer(
        &mut self,
        _ctx: &ElementValueArrayInitializerContext<'input>,
    ) {
        self.update_node_attrs(MetaType::ElementValueArrayInitializer, |node| {
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
            .push(Metadata::new(MetaType::AnnotationTypeDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#annotationTypeDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_annotationTypeDeclaration(&mut self, _ctx: &AnnotationTypeDeclarationContext<'input>) {
        self.update_node_attrs(MetaType::AnnotationTypeDeclaration, |node| {
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
            .push(Metadata::new(MetaType::AnnotationTypeBody));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#annotationTypeBody}.
     * @param ctx the parse tree
     */
    fn exit_annotationTypeBody(&mut self, _ctx: &AnnotationTypeBodyContext<'input>) {
        self.update_node_attrs(MetaType::AnnotationTypeBody, |node| {
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
            .push(Metadata::new(MetaType::AnnotationTypeElementDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#annotationTypeElementDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_annotationTypeElementDeclaration(
        &mut self,
        _ctx: &AnnotationTypeElementDeclarationContext<'input>,
    ) {
        self.update_node_attrs(MetaType::AnnotationTypeElementDeclaration, |node| {
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
            .push(Metadata::new(MetaType::AnnotationTypeElementRest));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#annotationTypeElementRest}.
     * @param ctx the parse tree
     */
    fn exit_annotationTypeElementRest(&mut self, _ctx: &AnnotationTypeElementRestContext<'input>) {
        self.update_node_attrs(MetaType::AnnotationTypeElementRest, |node| {
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
            .push(Metadata::new(MetaType::AnnotationMethodOrConstantRest));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#annotationMethodOrConstantRest}.
     * @param ctx the parse tree
     */
    fn exit_annotationMethodOrConstantRest(
        &mut self,
        _ctx: &AnnotationMethodOrConstantRestContext<'input>,
    ) {
        self.update_node_attrs(MetaType::AnnotationMethodOrConstantRest, |node| {
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
            .push(Metadata::new(MetaType::AnnotationMethodRest));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#annotationMethodRest}.
     * @param ctx the parse tree
     */
    fn exit_annotationMethodRest(&mut self, _ctx: &AnnotationMethodRestContext<'input>) {
        self.update_node_attrs(MetaType::AnnotationMethodRest, |node| {
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
            .push(Metadata::new(MetaType::AnnotationConstantRest));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#annotationConstantRest}.
     * @param ctx the parse tree
     */
    fn exit_annotationConstantRest(&mut self, _ctx: &AnnotationConstantRestContext<'input>) {
        self.update_node_attrs(MetaType::AnnotationConstantRest, |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#defaultValue}.
     * @param ctx the parse tree
     */
    fn enter_defaultValue(&mut self, _ctx: &DefaultValueContext<'input>) {
        self.stack_mut().push(Metadata::new(MetaType::DefaultValue));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#defaultValue}.
     * @param ctx the parse tree
     */
    fn exit_defaultValue(&mut self, _ctx: &DefaultValueContext<'input>) {
        self.update_node_attrs(MetaType::DefaultValue, |node| {
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
            .push(Metadata::new(MetaType::ModuleDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#moduleDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_moduleDeclaration(&mut self, _ctx: &ModuleDeclarationContext<'input>) {
        self.update_node_attrs(MetaType::ModuleDeclaration, |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#moduleBody}.
     * @param ctx the parse tree
     */
    fn enter_moduleBody(&mut self, _ctx: &ModuleBodyContext<'input>) {
        self.stack_mut().push(Metadata::new(MetaType::ModuleBody));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#moduleBody}.
     * @param ctx the parse tree
     */
    fn exit_moduleBody(&mut self, _ctx: &ModuleBodyContext<'input>) {
        self.update_node_attrs(MetaType::ModuleBody, |node| {
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
            .push(Metadata::new(MetaType::ModuleDirective));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#moduleDirective}.
     * @param ctx the parse tree
     */
    fn exit_moduleDirective(&mut self, _ctx: &ModuleDirectiveContext<'input>) {
        self.update_node_attrs(MetaType::ModuleDirective, |node| {
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
            .push(Metadata::new(MetaType::RequiresModifier));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#requiresModifier}.
     * @param ctx the parse tree
     */
    fn exit_requiresModifier(&mut self, _ctx: &RequiresModifierContext<'input>) {
        self.update_node_attrs(MetaType::RequiresModifier, |node| {
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
            .push(Metadata::new(MetaType::RecordDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#recordDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_recordDeclaration(&mut self, _ctx: &RecordDeclarationContext<'input>) {
        self.update_node_attrs(MetaType::RecordDeclaration, |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#recordHeader}.
     * @param ctx the parse tree
     */
    fn enter_recordHeader(&mut self, _ctx: &RecordHeaderContext<'input>) {
        self.stack_mut().push(Metadata::new(MetaType::RecordHeader));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#recordHeader}.
     * @param ctx the parse tree
     */
    fn exit_recordHeader(&mut self, _ctx: &RecordHeaderContext<'input>) {
        self.update_node_attrs(MetaType::RecordHeader, |node| {
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
            .push(Metadata::new(MetaType::RecordComponentList));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#recordComponentList}.
     * @param ctx the parse tree
     */
    fn exit_recordComponentList(&mut self, _ctx: &RecordComponentListContext<'input>) {
        self.update_node_attrs(MetaType::RecordComponentList, |node| {
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
            .push(Metadata::new(MetaType::RecordComponent));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#recordComponent}.
     * @param ctx the parse tree
     */
    fn exit_recordComponent(&mut self, _ctx: &RecordComponentContext<'input>) {
        self.update_node_attrs(MetaType::RecordComponent, |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#recordBody}.
     * @param ctx the parse tree
     */
    fn enter_recordBody(&mut self, _ctx: &RecordBodyContext<'input>) {
        self.stack_mut().push(Metadata::new(MetaType::RecordBody));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#recordBody}.
     * @param ctx the parse tree
     */
    fn exit_recordBody(&mut self, _ctx: &RecordBodyContext<'input>) {
        self.update_node_attrs(MetaType::RecordBody, |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#block}.
     * @param ctx the parse tree
     */
    fn enter_block(&mut self, _ctx: &BlockContext<'input>) {
        self.stack_mut().push(Metadata::new(MetaType::Block));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#block}.
     * @param ctx the parse tree
     */
    fn exit_block(&mut self, _ctx: &BlockContext<'input>) {
        self.update_node_attrs(MetaType::Block, |node| {
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
            .push(Metadata::new(MetaType::BlockStatement));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#blockStatement}.
     * @param ctx the parse tree
     */
    fn exit_blockStatement(&mut self, _ctx: &BlockStatementContext<'input>) {
        self.update_node_attrs(MetaType::BlockStatement, |node| {
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
            .push(Metadata::new(MetaType::LocalVariableDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#localVariableDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_localVariableDeclaration(&mut self, _ctx: &LocalVariableDeclarationContext<'input>) {
        self.update_node_attrs(MetaType::LocalVariableDeclaration, |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#identifier}.
     * @param ctx the parse tree
     */
    fn enter_identifier(&mut self, _ctx: &IdentifierContext<'input>) {
        self.stack_mut().push(Metadata::new(MetaType::Identifier));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#identifier}.
     * @param ctx the parse tree
     */
    fn exit_identifier(&mut self, _ctx: &IdentifierContext<'input>) {
        self.update_node_attrs(MetaType::Identifier, |node| {
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
            .push(Metadata::new(MetaType::TypeIdentifier));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#typeIdentifier}.
     * @param ctx the parse tree
     */
    fn exit_typeIdentifier(&mut self, _ctx: &TypeIdentifierContext<'input>) {
        self.update_node_attrs(MetaType::TypeIdentifier, |node| {
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
            .push(Metadata::new(MetaType::LocalTypeDeclaration));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#localTypeDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_localTypeDeclaration(&mut self, _ctx: &LocalTypeDeclarationContext<'input>) {
        self.update_node_attrs(MetaType::LocalTypeDeclaration, |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#statement}.
     * @param ctx the parse tree
     */
    fn enter_statement(&mut self, _ctx: &StatementContext<'input>) {
        self.stack_mut().push(Metadata::new(MetaType::Statement));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#statement}.
     * @param ctx the parse tree
     */
    fn exit_statement(&mut self, _ctx: &StatementContext<'input>) {
        self.update_node_attrs(MetaType::Statement, |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#catchClause}.
     * @param ctx the parse tree
     */
    fn enter_catchClause(&mut self, _ctx: &CatchClauseContext<'input>) {
        self.stack_mut().push(Metadata::new(MetaType::CatchClause));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#catchClause}.
     * @param ctx the parse tree
     */
    fn exit_catchClause(&mut self, _ctx: &CatchClauseContext<'input>) {
        self.update_node_attrs(MetaType::CatchClause, |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#catchType}.
     * @param ctx the parse tree
     */
    fn enter_catchType(&mut self, _ctx: &CatchTypeContext<'input>) {
        self.stack_mut().push(Metadata::new(MetaType::CatchType));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#catchType}.
     * @param ctx the parse tree
     */
    fn exit_catchType(&mut self, _ctx: &CatchTypeContext<'input>) {
        self.update_node_attrs(MetaType::CatchType, |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#finallyBlock}.
     * @param ctx the parse tree
     */
    fn enter_finallyBlock(&mut self, _ctx: &FinallyBlockContext<'input>) {
        self.stack_mut().push(Metadata::new(MetaType::FinallyBlock));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#finallyBlock}.
     * @param ctx the parse tree
     */
    fn exit_finallyBlock(&mut self, _ctx: &FinallyBlockContext<'input>) {
        self.update_node_attrs(MetaType::FinallyBlock, |node| {
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
            .push(Metadata::new(MetaType::ResourceSpecification));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#resourceSpecification}.
     * @param ctx the parse tree
     */
    fn exit_resourceSpecification(&mut self, _ctx: &ResourceSpecificationContext<'input>) {
        self.update_node_attrs(MetaType::ResourceSpecification, |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#resources}.
     * @param ctx the parse tree
     */
    fn enter_resources(&mut self, _ctx: &ResourcesContext<'input>) {
        self.stack_mut().push(Metadata::new(MetaType::Resources));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#resources}.
     * @param ctx the parse tree
     */
    fn exit_resources(&mut self, _ctx: &ResourcesContext<'input>) {
        self.update_node_attrs(MetaType::Resources, |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#resource}.
     * @param ctx the parse tree
     */
    fn enter_resource(&mut self, _ctx: &ResourceContext<'input>) {
        self.stack_mut().push(Metadata::new(MetaType::Resource));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#resource}.
     * @param ctx the parse tree
     */
    fn exit_resource(&mut self, _ctx: &ResourceContext<'input>) {
        self.update_node_attrs(MetaType::Resource, |node| {
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
            .push(Metadata::new(MetaType::SwitchBlockStatementGroup));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#switchBlockStatementGroup}.
     * @param ctx the parse tree
     */
    fn exit_switchBlockStatementGroup(&mut self, _ctx: &SwitchBlockStatementGroupContext<'input>) {
        self.update_node_attrs(MetaType::SwitchBlockStatementGroup, |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#switchLabel}.
     * @param ctx the parse tree
     */
    fn enter_switchLabel(&mut self, _ctx: &SwitchLabelContext<'input>) {
        self.stack_mut().push(Metadata::new(MetaType::SwitchLabel));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#switchLabel}.
     * @param ctx the parse tree
     */
    fn exit_switchLabel(&mut self, _ctx: &SwitchLabelContext<'input>) {
        self.update_node_attrs(MetaType::SwitchLabel, |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#forControl}.
     * @param ctx the parse tree
     */
    fn enter_forControl(&mut self, _ctx: &ForControlContext<'input>) {
        self.stack_mut().push(Metadata::new(MetaType::ForControl));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#forControl}.
     * @param ctx the parse tree
     */
    fn exit_forControl(&mut self, _ctx: &ForControlContext<'input>) {
        self.update_node_attrs(MetaType::ForControl, |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#forInit}.
     * @param ctx the parse tree
     */
    fn enter_forInit(&mut self, _ctx: &ForInitContext<'input>) {
        self.stack_mut().push(Metadata::new(MetaType::ForInit));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#forInit}.
     * @param ctx the parse tree
     */
    fn exit_forInit(&mut self, _ctx: &ForInitContext<'input>) {
        self.update_node_attrs(MetaType::ForInit, |node| {
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
            .push(Metadata::new(MetaType::EnhancedForControl));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#enhancedForControl}.
     * @param ctx the parse tree
     */
    fn exit_enhancedForControl(&mut self, _ctx: &EnhancedForControlContext<'input>) {
        self.update_node_attrs(MetaType::EnhancedForControl, |node| {
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
            .push(Metadata::new(MetaType::ParExpression));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#parExpression}.
     * @param ctx the parse tree
     */
    fn exit_parExpression(&mut self, _ctx: &ParExpressionContext<'input>) {
        self.update_node_attrs(MetaType::ParExpression, |node| {
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
            .push(Metadata::new(MetaType::ExpressionList));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#expressionList}.
     * @param ctx the parse tree
     */
    fn exit_expressionList(&mut self, _ctx: &ExpressionListContext<'input>) {
        self.update_node_attrs(MetaType::ExpressionList, |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#methodCall}.
     * @param ctx the parse tree
     */
    fn enter_methodCall(&mut self, _ctx: &MethodCallContext<'input>) {
        self.stack_mut().push(Metadata::new(MetaType::MethodCall));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#methodCall}.
     * @param ctx the parse tree
     */
    fn exit_methodCall(&mut self, _ctx: &MethodCallContext<'input>) {
        self.update_node_attrs(MetaType::MethodCall, |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#expression}.
     * @param ctx the parse tree
     */
    fn enter_expression(&mut self, _ctx: &ExpressionContext<'input>) {
        self.stack_mut().push(Metadata::new(MetaType::Expression));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#expression}.
     * @param ctx the parse tree
     */
    fn exit_expression(&mut self, _ctx: &ExpressionContext<'input>) {
        self.update_node_attrs(MetaType::Expression, |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#pattern}.
     * @param ctx the parse tree
     */
    fn enter_pattern(&mut self, _ctx: &PatternContext<'input>) {
        self.stack_mut().push(Metadata::new(MetaType::Pattern));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#pattern}.
     * @param ctx the parse tree
     */
    fn exit_pattern(&mut self, _ctx: &PatternContext<'input>) {
        self.update_node_attrs(MetaType::Pattern, |node| {
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
            .push(Metadata::new(MetaType::LambdaExpression));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#lambdaExpression}.
     * @param ctx the parse tree
     */
    fn exit_lambdaExpression(&mut self, _ctx: &LambdaExpressionContext<'input>) {
        self.update_node_attrs(MetaType::LambdaExpression, |node| {
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
            .push(Metadata::new(MetaType::LambdaParameters));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#lambdaParameters}.
     * @param ctx the parse tree
     */
    fn exit_lambdaParameters(&mut self, _ctx: &LambdaParametersContext<'input>) {
        self.update_node_attrs(MetaType::LambdaParameters, |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#lambdaBody}.
     * @param ctx the parse tree
     */
    fn enter_lambdaBody(&mut self, _ctx: &LambdaBodyContext<'input>) {
        self.stack_mut().push(Metadata::new(MetaType::LambdaBody));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#lambdaBody}.
     * @param ctx the parse tree
     */
    fn exit_lambdaBody(&mut self, _ctx: &LambdaBodyContext<'input>) {
        self.update_node_attrs(MetaType::LambdaBody, |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#primary}.
     * @param ctx the parse tree
     */
    fn enter_primary(&mut self, _ctx: &PrimaryContext<'input>) {
        self.stack_mut().push(Metadata::new(MetaType::Primary));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#primary}.
     * @param ctx the parse tree
     */
    fn exit_primary(&mut self, _ctx: &PrimaryContext<'input>) {
        self.update_node_attrs(MetaType::Primary, |node| {
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
            .push(Metadata::new(MetaType::SwitchExpression));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#switchExpression}.
     * @param ctx the parse tree
     */
    fn exit_switchExpression(&mut self, _ctx: &SwitchExpressionContext<'input>) {
        self.update_node_attrs(MetaType::SwitchExpression, |node| {
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
            .push(Metadata::new(MetaType::SwitchLabeledRule));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#switchLabeledRule}.
     * @param ctx the parse tree
     */
    fn exit_switchLabeledRule(&mut self, _ctx: &SwitchLabeledRuleContext<'input>) {
        self.update_node_attrs(MetaType::SwitchLabeledRule, |node| {
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
            .push(Metadata::new(MetaType::GuardedPattern));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#guardedPattern}.
     * @param ctx the parse tree
     */
    fn exit_guardedPattern(&mut self, _ctx: &GuardedPatternContext<'input>) {
        self.update_node_attrs(MetaType::GuardedPattern, |node| {
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
            .push(Metadata::new(MetaType::SwitchRuleOutcome));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#switchRuleOutcome}.
     * @param ctx the parse tree
     */
    fn exit_switchRuleOutcome(&mut self, _ctx: &SwitchRuleOutcomeContext<'input>) {
        self.update_node_attrs(MetaType::SwitchRuleOutcome, |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#classType}.
     * @param ctx the parse tree
     */
    fn enter_classType(&mut self, _ctx: &ClassTypeContext<'input>) {
        self.stack_mut().push(Metadata::new(MetaType::ClassType));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#classType}.
     * @param ctx the parse tree
     */
    fn exit_classType(&mut self, _ctx: &ClassTypeContext<'input>) {
        self.update_node_attrs(MetaType::ClassType, |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#creator}.
     * @param ctx the parse tree
     */
    fn enter_creator(&mut self, _ctx: &CreatorContext<'input>) {
        self.stack_mut().push(Metadata::new(MetaType::Creator));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#creator}.
     * @param ctx the parse tree
     */
    fn exit_creator(&mut self, _ctx: &CreatorContext<'input>) {
        self.update_node_attrs(MetaType::Creator, |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#createdName}.
     * @param ctx the parse tree
     */
    fn enter_createdName(&mut self, _ctx: &CreatedNameContext<'input>) {
        self.stack_mut().push(Metadata::new(MetaType::CreatedName));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#createdName}.
     * @param ctx the parse tree
     */
    fn exit_createdName(&mut self, _ctx: &CreatedNameContext<'input>) {
        self.update_node_attrs(MetaType::CreatedName, |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#innerCreator}.
     * @param ctx the parse tree
     */
    fn enter_innerCreator(&mut self, _ctx: &InnerCreatorContext<'input>) {
        self.stack_mut().push(Metadata::new(MetaType::InnerCreator));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#innerCreator}.
     * @param ctx the parse tree
     */
    fn exit_innerCreator(&mut self, _ctx: &InnerCreatorContext<'input>) {
        self.update_node_attrs(MetaType::InnerCreator, |node| {
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
            .push(Metadata::new(MetaType::ArrayCreatorRest));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#arrayCreatorRest}.
     * @param ctx the parse tree
     */
    fn exit_arrayCreatorRest(&mut self, _ctx: &ArrayCreatorRestContext<'input>) {
        self.update_node_attrs(MetaType::ArrayCreatorRest, |node| {
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
            .push(Metadata::new(MetaType::ClassCreatorRest));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#classCreatorRest}.
     * @param ctx the parse tree
     */
    fn exit_classCreatorRest(&mut self, _ctx: &ClassCreatorRestContext<'input>) {
        self.update_node_attrs(MetaType::ClassCreatorRest, |node| {
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
            .push(Metadata::new(MetaType::ExplicitGenericInvocation));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#explicitGenericInvocation}.
     * @param ctx the parse tree
     */
    fn exit_explicitGenericInvocation(&mut self, _ctx: &ExplicitGenericInvocationContext<'input>) {
        self.update_node_attrs(MetaType::ExplicitGenericInvocation, |node| {
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
            .push(Metadata::new(MetaType::TypeArgumentsOrDiamond));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#typeArgumentsOrDiamond}.
     * @param ctx the parse tree
     */
    fn exit_typeArgumentsOrDiamond(&mut self, _ctx: &TypeArgumentsOrDiamondContext<'input>) {
        self.update_node_attrs(MetaType::TypeArgumentsOrDiamond, |node| {
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
            .push(Metadata::new(MetaType::NonWildcardTypeArgumentsOrDiamond));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#nonWildcardTypeArgumentsOrDiamond}.
     * @param ctx the parse tree
     */
    fn exit_nonWildcardTypeArgumentsOrDiamond(
        &mut self,
        _ctx: &NonWildcardTypeArgumentsOrDiamondContext<'input>,
    ) {
        self.update_node_attrs(MetaType::NonWildcardTypeArgumentsOrDiamond, |node| {
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
            .push(Metadata::new(MetaType::NonWildcardTypeArguments));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#nonWildcardTypeArguments}.
     * @param ctx the parse tree
     */
    fn exit_nonWildcardTypeArguments(&mut self, _ctx: &NonWildcardTypeArgumentsContext<'input>) {
        self.update_node_attrs(MetaType::NonWildcardTypeArguments, |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#typeList}.
     * @param ctx the parse tree
     */
    fn enter_typeList(&mut self, _ctx: &TypeListContext<'input>) {
        self.stack_mut().push(Metadata::new(MetaType::TypeList));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#typeList}.
     * @param ctx the parse tree
     */
    fn exit_typeList(&mut self, _ctx: &TypeListContext<'input>) {
        self.update_node_attrs(MetaType::TypeList, |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#typeType}.
     * @param ctx the parse tree
     */
    fn enter_typeType(&mut self, _ctx: &TypeTypeContext<'input>) {
        self.stack_mut().push(Metadata::new(MetaType::TypeType));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#typeType}.
     * @param ctx the parse tree
     */
    fn exit_typeType(&mut self, _ctx: &TypeTypeContext<'input>) {
        self.update_node_attrs(MetaType::TypeType, |node| {
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
            .push(Metadata::new(MetaType::PrimitiveType));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#primitiveType}.
     * @param ctx the parse tree
     */
    fn exit_primitiveType(&mut self, _ctx: &PrimitiveTypeContext<'input>) {
        self.update_node_attrs(MetaType::PrimitiveType, |node| {
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
            .push(Metadata::new(MetaType::TypeArguments));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#typeArguments}.
     * @param ctx the parse tree
     */
    fn exit_typeArguments(&mut self, _ctx: &TypeArgumentsContext<'input>) {
        self.update_node_attrs(MetaType::TypeArguments, |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#superSuffix}.
     * @param ctx the parse tree
     */
    fn enter_superSuffix(&mut self, _ctx: &SuperSuffixContext<'input>) {
        self.stack_mut().push(Metadata::new(MetaType::SuperSuffix));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#superSuffix}.
     * @param ctx the parse tree
     */
    fn exit_superSuffix(&mut self, _ctx: &SuperSuffixContext<'input>) {
        self.update_node_attrs(MetaType::SuperSuffix, |node| {
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
            .push(Metadata::new(MetaType::ExplicitGenericInvocationSuffix));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#explicitGenericInvocationSuffix}.
     * @param ctx the parse tree
     */
    fn exit_explicitGenericInvocationSuffix(
        &mut self,
        _ctx: &ExplicitGenericInvocationSuffixContext<'input>,
    ) {
        self.update_node_attrs(MetaType::ExplicitGenericInvocationSuffix, |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
    /**
     * Enter a parse tree produced by {@link JavaParser#arguments}.
     * @param ctx the parse tree
     */
    fn enter_arguments(&mut self, _ctx: &ArgumentsContext<'input>) {
        self.stack_mut().push(Metadata::new(MetaType::Arguments));
    }
    /**
     * Exit a parse tree produced by {@link JavaParser#arguments}.
     * @param ctx the parse tree
     */
    fn exit_arguments(&mut self, _ctx: &ArgumentsContext<'input>) {
        self.update_node_attrs(MetaType::Arguments, |node| {
            node.set_attr(_ctx.get_text().as_str());
        });
        self.add_to_parent_member();
    }
}
